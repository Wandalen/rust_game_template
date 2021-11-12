mod common;

use common::*;

#[cfg_attr( target_os = "android", ndk_glue::main( backtrace = "on", logger( level = "debug", tag = "rust-app" ) ) )]
#[allow( dead_code )]
fn android_entry() 
{
  let event_loop = EventLoop::new();
  let window = winit::window::Window::new( &event_loop ).unwrap();

  pollster::block_on(run( event_loop, window ) );
}
