mod utils;
mod solver;

use std::char::MAX;

use wasm_bindgen::prelude::*;
pub use wasm_bindgen_rayon::init_thread_pool;

use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlBuffer};
use std::rc::{Rc};
use std::cell::{RefCell};


#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

static VERTEX_SHADER: &'static str = r#"
    varying highp vec3 vLighting;
        attribute vec2 aPosition;
        attribute vec4 aColor; 
        varying vec4 vColor; 
        uniform vec2 uResolution;

        void main() {
            vec2 position = (aPosition / uResolution) * 2.0 - 1.0;
            gl_Position = vec4(position, 0, 1);
            vColor = aColor; 
        }
"#;

static FRAGMENT_SHADER: &'static str = r#"
    precision mediump float;
    varying vec4 vColor; 
    void main() {
        gl_FragColor = vColor;
    }
"#;

pub struct Simulation {
    gl: WebGl2RenderingContext, 
    buffers: BufferPair, 
    state: solver::State, 
    mouse_info: MouseInfo, 
}

pub struct MouseInfo {
    mouse_x: Rc<RefCell<i32>>, 
    mouse_y: Rc<RefCell<i32>>,
    is_hovering: Rc<RefCell<bool>>,
}

pub struct BufferPair {
    color_buffer: WebGlBuffer, 
    position_buffer: WebGlBuffer, 
}

const NUM_PARTICLES: u32 = 12000;
const FIELD_HEIGHT: f32 = 1.5;
const FIELD_WIDTH: f32 = 1.5;
const VIEW_HEIGHT: u32 = 900;
const VIEW_WIDTH: u32 = (VIEW_HEIGHT as f32 * (FIELD_WIDTH / FIELD_HEIGHT)) as u32;
const SCALE: f32 = VIEW_HEIGHT as f32 / FIELD_HEIGHT;
const MAX_SPEED: f32 = 4.0;

impl Simulation {
    pub fn new(canvas: &web_sys::HtmlCanvasElement) -> Result<Simulation, JsValue> {
        let (gl, buffers) = init_webgl(canvas)?;
        let state = solver::State::new(NUM_PARTICLES, FIELD_HEIGHT, FIELD_WIDTH, SCALE);
        let mouse_info = MouseInfo::new(canvas)?;
        Ok(Simulation{ gl, buffers, state, mouse_info })
    }

    pub fn draw(&self) {
        self.gl.clear_color(0.4, 0.4, 0.4, 1.0);
        self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        let scale = VIEW_HEIGHT as f32 / FIELD_HEIGHT as f32;
        let positions = generate_positions(&self.state.particles, scale);
        self.gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.buffers.position_buffer));
        unsafe {
            self.gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER, 
                &js_sys::Float32Array::view(&positions), 
                WebGl2RenderingContext::DYNAMIC_DRAW
            );
        }
        self.gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.buffers.color_buffer));
        let colors = generate_colors(&self.state.particles);
        unsafe {
            self.gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER, 
                &js_sys::Float32Array::view(&colors), 
                WebGl2RenderingContext::DYNAMIC_DRAW
            );
        }
        self.gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 6 * self.state.num_particles as i32);
    }

    pub fn step(&mut self) {
        self.state.counter.store(0, std::sync::atomic::Ordering::SeqCst);
        for _ in 0..10 {
            self.state.step();
        }
        let s = format!("{:?}", self.state.counter);
        // log(&s);
    }
}

impl MouseInfo {
    pub fn new(canvas: &web_sys::HtmlCanvasElement) -> Result<MouseInfo, JsValue> {
        let mouse_x = Rc::new(RefCell::new(0));
        let mouse_y = Rc::new(RefCell::new(0));
        let is_hovering = Rc::new(RefCell::new(false));

        {
            let mouse_x = mouse_x.clone();
            let mouse_y = mouse_y.clone();
            add_event_listener(&canvas, "mousemove", move |event| {
                let mouse_event = event.dyn_into::<web_sys::MouseEvent>().unwrap();
                *mouse_x.borrow_mut() = mouse_event.offset_x();
                *mouse_y.borrow_mut() = mouse_event.offset_y();
            })?;
        }

        Ok(Self { mouse_x, mouse_y, is_hovering })
    }


}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    let canvas = get_canvas_element_by_id("canvas")?;
    let mut sim = Simulation::new(&canvas)?;

    start_animation(move||{
        sim.step();
        sim.draw();
    });
        
    Ok(())
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window().unwrap()
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
    let document = web_sys::window().unwrap().document().unwrap();
    document.get_element_by_id(id)
        .ok_or(JsValue::from("Element doesn't exist."))?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .or_else(|e| Err(JsValue::from(e)))
}

fn generate_positions(particles: &Vec<solver::Particle>, scale: f32) -> Vec<f32> {
    particles.iter().flat_map(|particle|{
        let x = particle.position.x * scale;
        let y = particle.position.y * scale;
        let size = particle.size;
        vec![
            x, y, 
            x + size, y, 
            x, y + size, 
            x, y + size, 
            x + size, y, 
            x + size, y + size
        ]
    }).collect()
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
 
fn generate_colors(particles: &Vec<solver::Particle>) -> Vec<f32> {
    particles.iter().flat_map(|particle|{
        let (r, g, b, a) = get_color_by_speed(particle.velocity.length());
        vec![
            r, g, b, a, 
            r, g, b, a, 
            r, g, b, a, 
            r, g, b, a, 
            r, g, b, a, 
            r, g, b, a  
        ]
    }).collect()
}

fn init_webgl(
    canvas: &web_sys::HtmlCanvasElement
) -> Result<(WebGl2RenderingContext, BufferPair), JsValue> {
    // set up canvas and webgl context handle
    canvas.set_height(VIEW_HEIGHT);
    canvas.set_width(VIEW_WIDTH);

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
        gl.vertex_attrib_pointer_with_i32(position_location as u32, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);
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
        gl.vertex_attrib_pointer_with_i32(position_location as u32, 4, WebGl2RenderingContext::FLOAT, false, 0, 0);
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