use webgpu::common::*;
use webgpu::pollster;

use webgpu::winit::
{
  event_loop::{ EventLoop },
  window::Window
};

pub fn main()
{
  let event_loop = EventLoop::new();
  let window = Window::new( &event_loop ).unwrap();

  // Temporarily avoid srgb formats for the swapchain on the web
  pollster::block_on(run( event_loop, window ) );
}

