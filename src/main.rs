extern crate glfw;

mod gl {
  include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

use glfw::{Action, Context, Key};
#[allow(unused_imports)]
use std::vec::Vec;
#[allow(unused_imports)]
use std::str;
#[allow(unused_imports)]
use gl::types::*;

fn main() {
  let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

  let (mut window, events) = glfw.create_window(
    500, 450,
    "Heav's Mandelbrot",
    glfw::WindowMode::Windowed
  ).unwrap();

  window.make_current();
  
  glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
  glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
  #[cfg(target_os = "macos")]
  glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
  
  window.set_key_polling(true);
  window.set_framebuffer_size_polling(true);

  let gl = gl::Gl::load_with(|ptr| window.get_proc_address(ptr));

  unsafe {
    let vs_src = b"
    #version 460
    precision mediump float;

    in vec2 a_position;
    out vec2 v_position;

    uniform float u_ratio;
    uniform float u_zoom;
    uniform vec2 u_position;

    void main() {
      gl_Position = vec4(a_position, 1.0, 1.0);
      v_position = a_position;
      v_position.x *= u_ratio;

      float zoom_factor = pow(0.75, u_zoom);

      v_position *= zoom_factor;
      v_position += u_position;
    }
    \0";
    let vs = gl.CreateShader(gl::VERTEX_SHADER);
    gl.ShaderSource(
      vs,
      1,
      [vs_src.as_ptr() as *const _].as_ptr(),
      std::ptr::null(),
    );
    gl.CompileShader(vs);

    let mut is_compiled = 0;
    gl.GetShaderiv(vs, gl::COMPILE_STATUS, &mut is_compiled);
    if is_compiled == 0 {
      let mut max_length = 0;
      gl.GetShaderiv(vs, gl::INFO_LOG_LENGTH, &mut max_length);

      let mut info_log: Vec<u8> = Vec::with_capacity(max_length as usize);
      gl.GetShaderInfoLog(vs, max_length, &mut max_length, info_log.as_mut_ptr() as *mut i8);
      info_log.set_len(max_length as usize);
      println!("{}", str::from_utf8(&info_log).unwrap());

      gl.DeleteShader(vs);
    }

    let fs_src = b"
    #version 460
    precision mediump float;

    in vec2 v_position;

    uniform float u_iterations;
    uniform float u_threshold;
    uniform float u_ramp;

    vec3 hsl2rgb( in vec3 c ){
      vec3 rgb = clamp( abs(mod(c.x*6.0+vec3(0.0,4.0,2.0),6.0)-3.0)-1.0, 0.0, 1.0 );
      return c.z + c.y * (rgb-0.5)*(1.0-abs(2.0*c.z-1.0));
    }

    vec3 color(float its) {
      float t = its/u_ramp;
      return hsl2rgb(vec3(t, 1., 0.5));
    }

    void main() {
      vec2 c = v_position.xy;
      vec2 z = vec2(0.);

      float iterations = 0.;
      for (float i = 0.; i < u_iterations; i++) {
        iterations = i;
        float zr2 = z.x * z.x;
        float zi2 = z.y * z.y;
    
        if(zr2 + zi2 > u_threshold) break;
        z = vec2(zr2 - zi2, 2.0 * z.x * z.y) + c;
      }

      gl_FragColor = vec4(color(iterations), 1.);
    }
    \0";
    let fs = gl.CreateShader(gl::FRAGMENT_SHADER);
    gl.ShaderSource(
      fs,
      1,
      [fs_src.as_ptr() as *const _].as_ptr(),
      std::ptr::null(),
    );
    gl.CompileShader(fs);

    let mut is_compiled = 0;
    gl.GetShaderiv(fs, gl::COMPILE_STATUS, &mut is_compiled);
    if is_compiled == 0 {
      let mut max_length = 0;
      gl.GetShaderiv(fs, gl::INFO_LOG_LENGTH, &mut max_length);

      let mut info_log: Vec<u8> = Vec::with_capacity(max_length as usize);
      gl.GetShaderInfoLog(fs, max_length, &mut max_length, info_log.as_mut_ptr() as *mut i8);
      info_log.set_len(max_length as usize);
      println!("{}", str::from_utf8(&info_log).unwrap());

      gl.DeleteShader(fs);
    }

    let program = gl.CreateProgram();
    gl.AttachShader(program, vs);
    gl.AttachShader(program, fs);

    gl.LinkProgram(program);

    let mut is_linked = 0;
    gl.GetProgramiv(program, gl::LINK_STATUS, &mut is_linked);
    if is_linked == 0 {
      let mut max_length = 0;
      gl.GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut max_length);

      let mut info_log: Vec<u8> = Vec::with_capacity(max_length as usize);
      gl.GetProgramInfoLog(program, max_length, &mut max_length, info_log.as_mut_ptr() as *mut i8);
      info_log.set_len(max_length as usize);
      println!("{}", str::from_utf8(&info_log).unwrap());

      gl.DeleteProgram(program);
      gl.DeleteShader(vs);
      gl.DeleteShader(fs);
    }

    gl.DetachShader(program, vs);
    gl.DetachShader(program, fs);

    gl.UseProgram(program);
    
    let vertices: Vec<f32> = vec![
      -1f32, 1f32, // 0
       1f32, 1f32, // 1
      -1f32,-1f32, // 3
       1f32, 1f32, // 1
       1f32,-1f32, // 2
      -1f32,-1f32, // 3
    ];
    let mut vertex_buffer = 0;
    gl.GenBuffers(1, &mut vertex_buffer);
    gl.BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);

    gl.BufferData(
      gl::ARRAY_BUFFER,
      (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
      vertices.as_ptr() as *const _,
      gl::STATIC_DRAW
    );

    if gl.BindVertexArray.is_loaded() {
      let mut vao = std::mem::zeroed();
      gl.GenVertexArrays(1, &mut vao);
      gl.BindVertexArray(vao);
    }

    let position_loc = gl.GetAttribLocation(program, b"a_position\0".as_ptr() as *const _);
    gl.EnableVertexAttribArray(position_loc as GLuint);

    gl.VertexAttribPointer(
      position_loc as GLuint,
      2,
      gl::FLOAT,
      0, // False
      2 * std::mem::size_of::<GLfloat>() as GLsizei,
      std::ptr::null()
    );

    gl.ClearColor(0.0,0.0,0.0,1.0);
    gl.UseProgram(program);

    /*
    VERTEX UNIFORMS
    */

    let ratio_loc = gl.GetUniformLocation(program, b"u_ratio\0".as_ptr() as *const GLchar);
    let zoom_loc = gl.GetUniformLocation(program, b"u_zoom\0".as_ptr() as *const GLchar);
    let position_loc = gl.GetUniformLocation(program, b"u_position\0".as_ptr() as *const GLchar);

    gl.Uniform1f(ratio_loc, 1f32);
    gl.Uniform1f(zoom_loc, 0f32);
    gl.Uniform2fv(position_loc, 2, [0.75, 0.0].as_ptr());
    
    /*
    FRAGMENT UNIFORMS
    */

    let iterations_loc = gl.GetUniformLocation(program, b"u_iterations\0".as_ptr() as *const GLchar);
    let threshold_loc = gl.GetUniformLocation(program, b"u_threshold\0".as_ptr() as *const GLchar);
    let ramp_loc = gl.GetUniformLocation(program, b"u_ramp\0".as_ptr() as *const GLchar);

    gl.Uniform1f(iterations_loc, 100f32);
    gl.Uniform1f(threshold_loc, 32f32);
    gl.Uniform1f(ramp_loc, 100f32);

    let mut err: GLenum = gl.GetError();
    while err != gl::NO_ERROR {
      println!("OpenGL Error {}", err);
      err = gl.GetError();
    }

    let mut zoom = 0f32;

    let delay = std::time::Duration::from_millis(100);
    while !window.should_close() {
      std::thread::sleep(delay);

      for (_, event) in glfw::flush_messages(&events) {
        match event {
          glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
          },
          glfw::WindowEvent::FramebufferSize(width, height) => {
            gl.Uniform1f(ratio_loc, (width as f32)/(height as f32));
            gl.Viewport(0, 0, width, height);
          },
          _ => {},
        }
      }
      glfw.poll_events();

      zoom += 0.25;
      gl.Uniform1f(zoom_loc, zoom);

      gl.Clear(gl::COLOR_BUFFER_BIT);
      gl.DrawArrays(gl::TRIANGLES, 0 as GLint, (vertices.len() as GLsizei)*(2 as GLsizei));
      window.swap_buffers();
    }
  }
}