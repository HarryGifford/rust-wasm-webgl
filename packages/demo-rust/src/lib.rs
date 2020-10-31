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
}

#[wasm_bindgen]
pub struct Context {
  pub width: u32,
  pub height: u32,
  pub mousex: Option<u32>,
  pub mousey: Option<u32>,
  program: Option<WebGlProgram>,
}

#[wasm_bindgen]
impl Context {
  #[wasm_bindgen(constructor)]
  pub fn new(width: u32, height: u32) -> Context {
    Context {
      width,
      height,
      mousex: None,
      mousey: None,
      program: None,
    }
  }

  pub fn dispose(&mut self, gl: &WebGl2RenderingContext) -> () {
    gl.delete_program(self.program.as_ref());
  }

  pub fn resize(&mut self, width: u32, height: u32) -> () {
    self.height = height;
    self.width = width;
    self.mousex = None;
    self.mousey = None;
    log(&format!("{} {}", width, height));
  }

  pub fn mousemove(&mut self, x: u32, y: u32, clicked: bool) -> () {
    if clicked {
      let (diffx, diffy) = match (self.mousex, self.mousey) {
        (None, None) => (0, 0),
        (Some(prevx), Some(prevy)) => ((x - prevx) as i32, (y - prevy) as i32),
        (_, _) => (0, 0),
      };
      self.mousex = Some(x);
      self.mousey = Some(y);
      log(&format!("{} {}", diffx, diffy));
    }
  }

  pub fn init_shaders(
    &mut self,
    gl: &WebGl2RenderingContext,
    vert_src: &str,
    frag_src: &str,
  ) -> Result<(), JsValue> {
    gl.delete_program(self.program.as_ref());
    let vert_shader = compile_shader(&gl, WebGl2RenderingContext::VERTEX_SHADER, vert_src)?;

    let frag_shader = compile_shader(&gl, WebGl2RenderingContext::FRAGMENT_SHADER, frag_src)?;
    self.program = Some(link_program(&gl, &vert_shader, &frag_shader)?);
    // One linked, we no longer need to keep the shader source around.
    gl.delete_shader(Some(&vert_shader));
    gl.delete_shader(Some(&frag_shader));
    Ok(())
  }

  pub fn render(
    &mut self,
    gl: &WebGl2RenderingContext,
    width: u32,
    height: u32,
  ) -> Result<(), JsValue> {
    self.resize(width, height);

    gl.use_program(self.program.as_ref());

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
