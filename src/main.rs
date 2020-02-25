extern crate glutin;

use glutin::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
  ContextBuilder
};
#[allow(unused_imports)]
use std::vec::Vec;
#[allow(unused_imports)]
use std::str;

mod gl {
  include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}
#[allow(unused_imports)]
use gl::types::*;

fn main() {
  let event_loop = EventLoop::new();
  let window_builder = WindowBuilder::new()
    .with_title("Heav's Mandelbrot");
  
  let windowed_context = ContextBuilder::new()
    .build_windowed(window_builder, &event_loop)
    .unwrap();
  let windowed_context = unsafe { windowed_context.make_current().unwrap() };

  let gl = gl::Gl::load_with(|ptr| windowed_context.context().get_proc_address(ptr) as *const _);

  unsafe {
    let vs_src = b"
    #version 100
    precision mediump float;

    attribute vec2 a_position;

    void main() {
      gl_Position = vec4(a_position, 1.0, 1.0);
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
    #version 100
    
    precision mediump float;

    void main() {
      gl_FragColor = vec4(1.,0.,0.,1.);
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
      -1f32,-1f32, // 3
       1f32, 1f32, // 2
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
      3,
      gl::FLOAT,
      0, // False
      0,
      0 as *const _
    );

    gl.UseProgram(program);
    
    gl.ClearColor(0.0,1.0,0.0,1.0);
    gl.Clear(gl::COLOR_BUFFER_BIT);
    gl.DrawArrays(gl::TRIANGLES, 0 as GLint, vertices.len() as GLsizei);

    let mut err: GLenum = gl.GetError();
    while err != gl::NO_ERROR {
      println!("OpenGL Error {}", err);
      err = gl.GetError();
    }

    event_loop.run(move |event, _, control_flow| {
      *control_flow = ControlFlow::Poll;

      match event {
        Event::WindowEvent {
          event,
          window_id: _,
        } => match event {
          WindowEvent::Resized(physical_size) => {
            windowed_context.resize(physical_size);
            gl.Viewport(0,0,500,500);
          }
          WindowEvent::CloseRequested => {
            *control_flow = ControlFlow::Exit
          }
          _ => (),
        },
        Event::MainEventsCleared => {
          windowed_context.window().request_redraw();
        }
        Event::RedrawRequested(_) => {
          gl.ClearColor(0.0,1.0,0.0,1.0);
          gl.Clear(gl::COLOR_BUFFER_BIT);
        }
        _ => (),
      }
    });
  }
}