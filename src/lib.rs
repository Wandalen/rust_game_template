mod common;

use common::*;
use mobile_entry_point::mobile_entry_point;

#[mobile_entry_point]
pub fn main() 
{
  let event_loop = EventLoop::new();
  let window = winit::window::Window::new(&event_loop).unwrap();

  // Temporarily avoid srgb formats for the swapchain on the web
  pollster::block_on(run(event_loop, window));
  // println!( "HELLO" );
}

#[cfg_attr( target_os = "android", ndk_glue::main( backtrace = "on", logger( level = "debug", tag = "rust-app" ) ) )]
#[allow( dead_code )]
fn android_entry() 
{
  let event_loop = EventLoop::new();
  let window = winit::window::Window::new( &event_loop ).unwrap();

  pollster::block_on(run( event_loop, window ) );
}


 