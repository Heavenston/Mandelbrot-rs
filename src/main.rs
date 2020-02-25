extern crate glutin;

use glutin::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
  ContextBuilder
};
use std::vec::Vec;
use std::str;

mod gl {
  include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}
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

    /*let mut success: GLint = 0;
    gl::GetShaderiv(vs, gl::COMPILE_STATUS, &mut success);
    println!("Success state : {}", success);
    if success == 0 {
      let mut max_length = 0;
      gl::GetShaderiv(vs, gl::INFO_LOG_LENGTH, &mut max_length);
      println!("max_length : {}", max_length);

      let mut error_log: Vec<u8> = Vec::with_capacity(max_length as usize);
      gl::GetShaderInfoLog(
        vs,
        max_length,
        &mut max_length,
        error_log.as_mut_ptr() as *mut i8
      );
      error_log.set_len(max_length as usize);
      println!("Info log : {:s}", str::from_utf8(&error_log).unwrap());
    }*/
  }

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Poll;

    match event {
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        window_id,
      } if window_id == windowed_context.window().id() => *control_flow = ControlFlow::Exit,
      Event::MainEventsCleared => {
        windowed_context.window().request_redraw();
      },
      Event::RedrawRequested(_) => {
        
      },
      _ => (),
    }
  });
}