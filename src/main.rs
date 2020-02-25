extern crate glutin;

use glutin::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
  ContextBuilder
};

mod gl {
  include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

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
    let vs_src = "attribute vec2 a_position\nvoid main() {\ngl_Position = vec4(a_position, 1.0, 1.0);\n}";
    let vs = gl::CreateShader(gl::VERTEX_SHADER);
    gl::ShaderSource(
      vs,
      1,
      [vs_src.as_ptr() as *const _].as_ptr(),
      std::ptr::null(),
    );
    gl::CompileShader(vs);

    let mut err = gl::GetError();
    while err != gl::NO_ERROR {
      println!("Error: {}", err);
      err = gl::GetError()
    }
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