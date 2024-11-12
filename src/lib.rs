mod utils;
mod solver;

use wasm_bindgen::prelude::*;
pub use wasm_bindgen_rayon::init_thread_pool;

use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlBuffer};


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

#[wasm_bindgen]
pub struct Simulation {
    gl: WebGl2RenderingContext, 
    buffers: BufferPair, 
    state: solver::State, 
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

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(
        canvas: &web_sys::OffscreenCanvas
    ) -> Result<Simulation, JsValue> {
        let (gl, buffers) = init_webgl(canvas)?;
        let state = solver::State::new(NUM_PARTICLES, FIELD_HEIGHT, FIELD_WIDTH, SCALE);
        Ok(Simulation{ gl, buffers, state })
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
        log(&s);
    }
}

pub fn generate_positions(particles: &Vec<solver::Particle>, scale: f32) -> Vec<f32> {
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

pub fn generate_colors(particles: &Vec<solver::Particle>) -> Vec<f32> {
    particles.iter().flat_map(|particle|{
        let r = 0.0;
        let g = 0.0;
        let b = 1.0;
        let a = 1.0;
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
    canvas: &web_sys::OffscreenCanvas
) -> Result<(WebGl2RenderingContext, BufferPair), JsValue> {
    // set up canvas and webgl context handle
    canvas.set_width(VIEW_HEIGHT);
    canvas.set_height(VIEW_WIDTH);

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