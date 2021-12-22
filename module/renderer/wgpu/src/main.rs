#![ warn( missing_docs ) ]

mod common;

use crate::common::Renderer;

#[cfg( target_arch = "wasm32" )]
use winit::platform::web::WindowExtWebSys;

// qqq : implement release build

struct App;

impl common::Renderer for App
{
  fn new() -> App
  {
    App{}
  }

  #[cfg( target_arch = "wasm32" )]
  fn run( &self ) -> ()
  {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::Window::new( &event_loop ).unwrap();

    console_log::init_with_level( log::Level::Warn ).expect( "Could not initialize logger" );

    #[ cfg( debug ) ]
    std::panic::set_hook( Box::new( console_error_panic_hook::hook ) );

    // panic!( "abc!" );

    web_sys::window()
    .and_then( | win | win.document() )
    .and_then( | doc | doc.body() )
    .and_then( | body | body.append_child( &web_sys::Element::from( window.canvas() ) ).ok() )
    .expect( "Couldn't append canvas to document body" );
    wasm_bindgen_futures::spawn_local( common::run( event_loop, window ) );
  }
}

pub fn main()
{
  println!( "xxx" );
  let app = App::new();
  app.run();
}

#[cfg_attr( target_os = "ios", mobile_entry_point::mobile_entry_point ) ]
pub fn ios_main()
{
  main();
}

#[cfg_attr( target_os = "android", ndk_glue::main( backtrace = "on" ) ) ]
pub fn android_main()
{
  main();
}
