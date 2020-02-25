extern crate winit;
extern crate glutin;

use glutin::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
  ContextBuilder
};

fn main() {
  let event_loop = EventLoop::new();
  let window_builder = WindowBuilder::new()
    .with_title("Heav's Mandelbrot");
  
  let windowed_context = ContextBuilder::new()
    .build_windowed(window_builder, &event_loop)
    .unwrap();
  let windowed_context = unsafe { windowed_context.make_current().unwrap() };

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