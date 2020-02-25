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

  gl::load_with(|ptr| windowed_context.context().get_proc_address(ptr) as *const _);

  
  unsafe {
    let vs_src = "attribute vec2 a_position;\nvoid main() {\ngl_Position = vec4(a_position, 1.0, 1.0);\n}";
    let vs = gl::CreateShader(gl::VERTEX_SHADER);
    gl::ShaderSource(
      vs,
      1,
      [vs_src.as_ptr() as *const _].as_ptr(),
      std::ptr::null(),
    );
    gl::CompileShader(vs);

    let fs_src = "precision mediump float;\nvoid main() {\ngl_FragColor = vec4(1.,0.,0.,1.)\n}";
    let fs = gl::CreateShader(gl::VERTEX_SHADER);
    gl::ShaderSource(
      fs,
      1,
      [fs_src.as_ptr() as *const _].as_ptr(),
      std::ptr::null(),
    );
    gl::CompileShader(fs);

    let program = gl::CreateProgram();
    gl::AttachShader(program, vs);
    gl::AttachShader(program, fs);

    gl::LinkProgram(program);

    gl::DetachShader(program, vs);
    gl::DetachShader(program, fs);

    gl::UseProgram(program);
    
    let vertices: Vec<f32> = vec![
      -1f32, 1f32, // 0
       1f32, 1f32, // 1
      -1f32,-1f32, // 3
       1f32, 1f32, // 1
      -1f32,-1f32, // 3
       1f32, 1f32, // 2
    ];
    let mut vertex_buffer = 0;
    gl::GenBuffers(1, &mut vertex_buffer);
    gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);

    gl::BufferData(
      gl::ARRAY_BUFFER,
      (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
      vertices.as_ptr() as *const _,
      gl::STATIC_DRAW
    );

    if gl::BindVertexArray::is_loaded() {
      let mut vao = std::mem::zeroed();
      gl::GenVertexArrays(1, &mut vao);
      gl::BindVertexArray(vao);
    }

    let position_loc = gl::GetAttribLocation(program, b"a_position\0".as_ptr() as *const _);
    gl::EnableVertexAttribArray(position_loc as GLuint);

    gl::VertexAttribPointer(
      position_loc as GLuint,
      3,
      gl::FLOAT,
      0, // False
      0,
      0 as *const _
    );

    event_loop.run(move |event, _, control_flow| {
      *control_flow = ControlFlow::Poll;

      match event {
        Event::WindowEvent {
          event,
          window_id: _,
        } => match event {
          WindowEvent::Resized(physical_size) => {
            windowed_context.resize(physical_size)
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
          gl::DrawArrays(gl::TRIANGLES, 0 as GLint, vertices.len() as GLsizei);
        }
        _ => (),
      }
    });
  }
}