pub mod common;

pub use wgpu;
pub use winit;
pub use pollster;

pub use self::common::*;

pub trait Renderer
{
  fn new() -> Self;
  
  fn run ( &self ) -> ()
  {
    let event_loop = EventLoop::new();
    let window = Window::new( &event_loop ).unwrap();
    pollster::block_on(run( event_loop, window ) );
  }
}

