mod utils;
mod solver;

use core::num;
use std::char::MAX;

use wasm_bindgen::prelude::*;
pub use wasm_bindgen_rayon::init_thread_pool;
use web_time::Instant;

use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlShader, Window};
use std::rc::Rc;
use std::cell::RefCell;


#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

static VERTEX_SHADER: &'static str = r#"
    varying highp vec3 vLighting;
    attribute vec3 aPosition;
    attribute vec3 aColor; 
    varying vec3 vColor; 
    uniform vec2 uResolution;

    void main() {
        vec2 position = (aPosition.xy / uResolution) * 2.0 - 1.0;
        gl_Position = vec4(position, 0, 1);
        float radius = aPosition.z * 0.8;
        gl_PointSize = radius * 2.0;
        vColor = aColor; 
    }
"#;

static FRAGMENT_SHADER: &'static str = r#"
    precision mediump float;
    varying vec3 vColor; 
    void main() {
        lowp vec2 pos = gl_PointCoord - vec2(0.5, 0.5);
		lowp float dist_squared = dot(pos, pos);
        lowp float alpha;

        if (dist_squared < 0.25) {
            alpha = 1.0;
        } else {
            alpha = 0.0;
        }

        gl_FragColor = vec4(vColor, alpha);
    }
"#;

pub struct Simulation {
    gl: WebGl2RenderingContext, 
    buffers: BufferPair, 
    state: solver::State, 
    mouse_info: MouseInfo, 
    button_pressed: Rc<RefCell<bool>>, 
    window_size: WindowSize, 
    scale: f32, 
}

#[derive(Debug)]
pub struct MouseInfo {
    mouse_x: Rc<RefCell<f32>>, 
    mouse_y: Rc<RefCell<f32>>,
    is_dragging: Rc<RefCell<bool>>,
}

struct BufferPair {
    color_buffer: WebGlBuffer, 
    position_buffer: WebGlBuffer, 
}

struct WindowSize {
    width: f32, 
    height: f32, 
}

const MAX_SPEED: f32 = 4.0;

macro_rules! benchmark {
    ($code:block) => {{
        let start = Instant::now(); 
        $code
        start.elapsed().as_micros()
    }};
}

impl Simulation {
    pub fn new(canvas: &web_sys::HtmlCanvasElement, num_particles: u32) -> Result<Simulation, JsValue> {
        let window_size = get_window_size()?;
        let scale = window_size.height / solver::State::height_from_num_particles(num_particles);
        let (gl, buffers) = init_webgl(canvas, &window_size)?;
        let aspect_ratio = window_size.width / window_size.height;
        let state = solver::State::new(num_particles, aspect_ratio);
        let button_pressed = init_button_info()?;
        let mouse_info = MouseInfo::new(canvas)?;
        Ok(Simulation{ gl, buffers, state, mouse_info, button_pressed, window_size, scale })
    }

    pub fn draw(&self) {
        self.gl.clear_color(0.4, 0.4, 0.4, 1.0);
        self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        // アルファ値の設定のために必要らしい？（TODO : 調べる）
        self.gl.enable(WebGl2RenderingContext::BLEND);
        self.gl.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);

        let positions = self.generate_positions();
        self.gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.buffers.position_buffer));
        unsafe {
            self.gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER, 
                &js_sys::Float32Array::view(&positions), 
                WebGl2RenderingContext::DYNAMIC_DRAW
            );
        }
        self.gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.buffers.color_buffer));
        let colors = self.generate_colors();
        unsafe {
            self.gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER, 
                &js_sys::Float32Array::view(&colors), 
                WebGl2RenderingContext::DYNAMIC_DRAW
            );
        }
        self.gl.draw_arrays(WebGl2RenderingContext::POINTS, 0, self.state.particles.len() as i32);
    }

    fn reset(&mut self, num_particles: u32) {
        self.state.init_particles(num_particles, self.window_size.width / self.window_size.height);
        self.scale = self.window_size.height / solver::State::height_from_num_particles(num_particles);
        *self.button_pressed.borrow_mut() = false;
    }

    fn generate_positions(&self) -> Vec<f32> {
        self.state.particles.iter().flat_map(|particle|{
            let x = particle.position.x * self.scale;
            let y = particle.position.y * self.scale;
            let r = solver::PARTICLE_SIZE * self.scale;
            vec![ x, y, r ]
        }).collect()
    }

    fn generate_colors(&self) -> Vec<f32> {
        self.state.particles.iter().flat_map(|particle|{
            let (r, g, b, a) = get_color_by_speed(particle.velocity.length());
            vec![ r, g, b ]
        }).collect()
    }

    pub fn step(&mut self) {
        let mouse_vec = glam::Vec2::new(*self.mouse_info.mouse_x.borrow() / self.scale, self.state.field.height - *self.mouse_info.mouse_y.borrow() / self.scale);
        let t = benchmark!({self.state.update(mouse_vec, *self.mouse_info.is_dragging.borrow())});
        let s = format!("{} ms", t / 1000);
        log(&s);
    }
}

impl MouseInfo {
    pub fn new(canvas: &web_sys::HtmlCanvasElement) -> Result<MouseInfo, JsValue> {
        let mouse_x = Rc::new(RefCell::new(0.0));
        let mouse_y = Rc::new(RefCell::new(0.0));
        let is_dragging = Rc::new(RefCell::new(false));

        {
            let mouse_x = mouse_x.clone();
            let mouse_y = mouse_y.clone();
            add_event_listener(&canvas, "mousemove", move |event| {
                let mouse_event = event.dyn_into::<web_sys::MouseEvent>().unwrap();
                *mouse_x.borrow_mut() = mouse_event.offset_x() as f32;
                *mouse_y.borrow_mut() = mouse_event.offset_y() as f32;
            })?;
        }

        {
            let is_dragging = is_dragging.clone();
            add_event_listener(&canvas, "mouseleave", move|event|{
                *is_dragging.borrow_mut() = false;
            })?;
        }

        {
            let is_dragging = is_dragging.clone();
            add_event_listener(&canvas, "mouseup", move|event|{
                *is_dragging.borrow_mut() = false;
            })?;
        }

        {
            let is_dragging = is_dragging.clone();
            add_event_listener(&canvas, "mousedown", move|event|{
                *is_dragging.borrow_mut() = true;
            })?;
        }

        Ok(Self { mouse_x, mouse_y, is_dragging })
    }


}

fn init_button_info() -> Result<Rc<RefCell<bool>>, JsValue> {
    let button_pressed = Rc::new(RefCell::new(false));
    let document = window().document().unwrap();
    let button_element = document.get_element_by_id("reset-button").ok_or(JsValue::from("reset-button doesn't exist"))?;
    {
        let button_pressed_clone = button_pressed.clone();
        add_event_listener(&button_element, "click", move |event| {
            *button_pressed_clone.borrow_mut() = true;
        });
    }
    Ok(button_pressed)
}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    let canvas = get_canvas_element_by_id("canvas")?;
    let num_particles = get_particle_count().unwrap();
    let mut sim = Simulation::new(&canvas, num_particles)?;

    start_animation(move||{
        if *sim.button_pressed.borrow() {
            let num_particles = get_particle_count().unwrap();
            sim.reset(num_particles);
        }
        sim.step();
        sim.draw();
    });
        
    Ok(())
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn get_window_size() -> Result<WindowSize, JsValue> {
    let window = window();
    let width = window.inner_width()?.as_f64().unwrap() as f32;
    let height = window.inner_height()?.as_f64().unwrap() as f32;
    Ok(WindowSize { width, height })
}

fn get_particle_count() -> Result<u32, JsValue> {
    let document = window().document().unwrap();
    let element = document.get_element_by_id("slider-value")
        .ok_or(JsValue::from("slider-value doesn't exist."))?;
    let text_content = element.text_content().unwrap();
    Ok(text_content.trim().parse::<u32>().unwrap())
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn start_animation<T>(mut handler: T)
where T: 'static + FnMut()
{ 
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        handler();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn get_canvas_element_by_id(id: &str) -> Result<web_sys::HtmlCanvasElement, JsValue> {
    let document = window().document().unwrap();
    document.get_element_by_id(id)
        .ok_or(JsValue::from("Element doesn't exist."))?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .or_else(|e| Err(JsValue::from(e)))
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32, f32) {
    let i = (h * 6.0).floor() as u32;
    let f = h * 6.0 - i as f32;
    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);

    let (mut r, mut g, mut b) = (0.0, 0.0, 0.0);
    
    match i % 6 {
        0 => { r = v; g = t; b = p; }
        1 => { r = q; g = v; b = p; }
        2 => { r = p; g = v; b = t; }
        3 => { r = p; g = q; b = v; }
        4 => { r = t; g = p; b = v; }
        5 => { r = v; g = p; b = q; }
        _ => {}
    }

    (r, g, b, 1.0)
}

fn get_color_by_speed(speed: f32) -> (f32, f32, f32, f32) {
    let normalized_speed = (speed.abs() / MAX_SPEED).min(1.0);
    let hue = (1.0 - normalized_speed) * 0.7;
    let saturation = 1.0;
    let value = 1.0;
    hsv_to_rgb(hue, saturation, value)
}

fn init_webgl(
    canvas: &web_sys::HtmlCanvasElement, 
    window_size: &WindowSize, 
) -> Result<(WebGl2RenderingContext, BufferPair), JsValue> {
    canvas.set_height(window_size.height as u32);
    canvas.set_width(window_size.width as u32);

    let gl = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let shader_program = init_shader_program(&gl)?;

    gl.clear_color(0.4, 0.4, 0.4, 1.0); 
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    let position_buffer = gl.create_buffer().unwrap();
    let color_buffer = gl.create_buffer().unwrap();

    set_position_attribute(&gl, &shader_program, &position_buffer)?;
    set_color_attribute(&gl, &shader_program, &color_buffer)?;

    let resolution_location = gl.get_uniform_location(&shader_program, "uResolution").unwrap();
    gl.uniform2f(Some(&resolution_location), canvas.width() as f32, canvas.height() as f32);

    Ok((gl, BufferPair{ position_buffer, color_buffer }))
}

fn init_shader_program(
    gl: &WebGl2RenderingContext, 
) -> Result<WebGlProgram, JsValue> {
    let vertex_shader = compile_shader(&gl, WebGl2RenderingContext::VERTEX_SHADER, VERTEX_SHADER)?;
    let fragment_shader = compile_shader(&gl, WebGl2RenderingContext::FRAGMENT_SHADER, FRAGMENT_SHADER)?;

    let program = gl.create_program().unwrap();
    gl.attach_shader(&program, &vertex_shader);
    gl.attach_shader(&program, &fragment_shader);
    gl.link_program(&program);
    gl.use_program(Some(&program));

    Ok(program)
}

fn compile_shader(
    gl: &WebGl2RenderingContext, 
    shader_type: u32, 
    source: &str
) -> Result<WebGlShader, JsValue> {
    let shader = gl.create_shader(shader_type).unwrap();
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    let success = gl
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false);

    if !success {
        let error_msg = gl.get_shader_info_log(&shader).unwrap_or_else(|| "Unknown error".into());
        return Err(JsValue::from_str(&error_msg));
    }

    Ok(shader)
}

fn set_position_attribute(
    gl: &WebGl2RenderingContext, 
    program: &WebGlProgram, 
    buffer: &WebGlBuffer
) -> Result<(), JsValue>{
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    let position_location = gl.get_attrib_location(program, "aPosition");

    if position_location >= 0 {
        gl.vertex_attrib_pointer_with_i32(position_location as u32, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(position_location as u32);
    } else {
        return Err(JsValue::from_str("cannot set position attribute"));
    }

    return Ok(());
}

fn set_color_attribute(
    gl: &WebGl2RenderingContext, 
    program: &WebGlProgram, 
    buffer: &WebGlBuffer
) -> Result<(), JsValue>{
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    let position_location = gl.get_attrib_location(program, "aColor");

    if position_location >= 0 {
        gl.vertex_attrib_pointer_with_i32(position_location as u32, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(position_location as u32);
    } else {
        return Err(JsValue::from_str("cannot set color attribute"));
    }

    return Ok(());
}


fn add_event_listener<T>(target: &web_sys::Element, event_name: &str, handler: T) -> Result<(), JsValue>
where
    T: 'static + FnMut(web_sys::Event)
{
    let cb = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    target.add_event_listener_with_callback(event_name, cb.as_ref().unchecked_ref())?;
    cb.forget();

    Ok(())
}