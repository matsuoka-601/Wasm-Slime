mod utils;

pub use wasm_bindgen_rayon::init_thread_pool;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use wasm_bindgen::prelude::*;

use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn sum(input: &[i32]) -> i32 {
    input.par_iter().map(|&x| x).sum()
    // return 0;
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
    gl: WebGl2RenderingContext
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(
        canvas: &web_sys::OffscreenCanvas
    ) -> Result<Simulation, JsValue> {
        let gl = init_webgl(canvas)?;
        Ok(Simulation { gl })
    }
}

fn init_webgl(
    canvas: &web_sys::OffscreenCanvas,
) -> Result<WebGl2RenderingContext, JsValue> {
    // set up canvas and webgl context handle
    canvas.set_width(300);
    canvas.set_height(300);

    let gl = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let program = init_shader_program(&gl)?;

    gl.clear_color(0.4, 0.4, 0.4, 1.0); // 背景を黒に設定
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    Ok(gl)
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