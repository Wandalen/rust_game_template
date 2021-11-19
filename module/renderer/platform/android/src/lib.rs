use webgpu::common::*;
use webgpu::pollster;

use webgpu::winit::
{
  event_loop::{ EventLoop },
  window::Window
};

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
pub fn main()
{
  let event_loop = EventLoop::new();
  let window = Window::new( &event_loop ).unwrap();
  pollster::block_on(run(event_loop, window));
}