mod utils;

use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);

  fn alert(s: &str);
}

struct Context {
  width: u32,
  height: u32,
  mousePos: Option<[u32; 2]>,
}

static mut CONTEXT: Context = Context {
  width: 0,
  height: 0,
  mousePos: None,
};

#[wasm_bindgen]
pub fn resize(width: u32, height: u32) -> () {
  unsafe {
    CONTEXT = Context {
      width,
      height,
      mousePos: None,
    }
  }
  log(&format!("{} {}", width, height));
}

#[wasm_bindgen]
pub fn mousemove(x: u32, y: u32, clicked: bool) -> () {
  if clicked {
    let (diffx, diffy) = unsafe {
      match CONTEXT.mousePos {
        None => (0, 0),
        Some([prevx, prevy]) => ((x - prevx) as i32, (y - prevy) as i32),
      }
    };
    unsafe {
      CONTEXT = Context {
        mousePos: Some([x, y]),
        ..CONTEXT
      }
    }
    log(&format!("{} {}", diffx, diffy));
  }
}

#[wasm_bindgen]
pub fn render(gl: WebGl2RenderingContext, width: u32, height: u32) -> Result<(), JsValue> {
  resize(width, height);

  let vert_shader = compile_shader(
    &gl,
    WebGl2RenderingContext::VERTEX_SHADER,
    r#"#version 300 es
        precision highp float;
        precision highp int;
        in vec4 position;
        out vec4 coord;
        void main() {
            gl_Position = position;
            coord = position;
        }
    "#,
  )?;

  let frag_shader = compile_shader(
    &gl,
    WebGl2RenderingContext::FRAGMENT_SHADER,
    r#"#version 300 es
        precision highp float;
        precision highp int;
        in vec4 coord;
        out vec4 color;
        vec4 invGamma(vec4 color) {
            return vec4(pow(color.rgb, vec3(1.0/2.2)), color.a);
        }
        vec4 gamma(vec4 color) {
            return vec4(pow(color.rgb, vec3(2.2)), color.a);
        }
        // Taken from https://en.wikipedia.org/wiki/Mandelbrot_set
        float iter(vec2 pos) {
            vec2 x = pos;
            int iters = 0;
            int max_iters = 100;
            while (x.x*x.x + x.y*x.y < 2.0 && iters < max_iters) {
                float xt = x.x*x.x - x.y*x.y + pos.x;
                x.y = 2.0 * x.x * x.y + pos.y;
                x.x = xt;
                iters++;
            }
            return float(iters)/float(max_iters);
        }
        void main() {
            color = vec4(vec3(iter(coord.xy)), 1.0);
        }
    "#,
  )?;
  let program = link_program(&gl, &vert_shader, &frag_shader)?;
  gl.use_program(Some(&program));

  let vertices: [f32; 12] = [
    -1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0, 1.0, 0.0, -1.0, 1.0, 0.0,
  ];
  let indices: [u16; 6] = [0, 1, 3, 2, 3, 1];

  let buffer = gl.create_buffer().ok_or("failed to create buffer")?;
  gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

  let ind_buf = gl.create_buffer().ok_or("failed to create buffer")?;
  gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&ind_buf));

  // Note that `Float32Array::view` is somewhat dangerous (hence the
  // `unsafe`!). This is creating a raw view into our module's
  // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
  // (aka do a memory allocation in Rust) it'll cause the buffer to change,
  // causing the `Float32Array` to be invalid.
  //
  // As a result, after `Float32Array::view` we have to be very careful not to
  // do any memory allocations before it's dropped.
  unsafe {
    let vert_array = js_sys::Float32Array::view(&vertices);
    gl.buffer_data_with_array_buffer_view(
      WebGl2RenderingContext::ARRAY_BUFFER,
      &vert_array,
      WebGl2RenderingContext::STATIC_DRAW,
    );

    let ind_array = js_sys::Uint16Array::view(&indices);
    gl.buffer_data_with_array_buffer_view(
      WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
      &ind_array,
      WebGl2RenderingContext::STATIC_DRAW,
    );
  }
  gl.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
  gl.enable_vertex_attrib_array(0);

  gl.clear_color(0.0, 0.0, 0.0, 1.0);
  gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

  gl.draw_elements_with_i32(
    WebGl2RenderingContext::TRIANGLES,
    indices.len() as i32,
    WebGl2RenderingContext::UNSIGNED_SHORT,
    0,
  );
  Ok(())
}

pub fn compile_shader(
  gl: &WebGl2RenderingContext,
  shader_type: u32,
  source: &str,
) -> Result<WebGlShader, String> {
  let shader = gl
    .create_shader(shader_type)
    .ok_or_else(|| String::from("Unable to create shader object"))?;
  gl.shader_source(&shader, source);
  gl.compile_shader(&shader);

  if gl
    .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
    .as_bool()
    .unwrap_or(false)
  {
    Ok(shader)
  } else {
    Err(
      gl.get_shader_info_log(&shader)
        .unwrap_or_else(|| String::from("Unknown error creating shader")),
    )
  }
}

pub fn link_program(
  gl: &WebGl2RenderingContext,
  vert_shader: &WebGlShader,
  frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
  let program = gl
    .create_program()
    .ok_or_else(|| String::from("Unable to create shader object {}"))?;

  gl.attach_shader(&program, vert_shader);
  gl.attach_shader(&program, frag_shader);
  gl.link_program(&program);

  if gl
    .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
    .as_bool()
    .unwrap_or(false)
  {
    Ok(program)
  } else {
    Err(
      gl.get_program_info_log(&program)
        .unwrap_or_else(|| String::from("Unknown error creating program object")),
    )
  }
}
