#[cfg( target_os = "android" )]

use webgpu::common::*;
use webgpu::pollster;

use webgpu::winit::
{
  event_loop::{ EventLoop },
  window::Window
};

#[cfg_attr( ndk_glue::main( backtrace = "on", logger( level = "debug", tag = "rust-app" ) ) )]
fn main()
{
  let event_loop = EventLoop::new();
  let window = Window::new( &event_loop ).unwrap();
  pollster::block_on(run(event_loop, window));
}