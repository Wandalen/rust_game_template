#[cfg( target_arch = "wasm32" )]

use game_template::
{
  // common::*,
  Renderer,
};

use game_template::winit::
{
  event_loop::{ EventLoop },
  window::Window,
  platform::web::WindowExtWebSys
};

struct Web;

impl game_template::Renderer for Web
{

  fn new() -> Web
  {
    Web{}
  }

  fn run( &self ) -> ()
  {
    let event_loop = EventLoop::new();
    let window = Window::new( &event_loop ).unwrap();

    std::panic::set_hook( Box::new( console_error_panic_hook::hook ) );
    console_log::init().expect( "could not initialize logger" );
    // On wasm, append the canvas to the document body
    web_sys::window()
    .and_then( | win | win.document() )
    .and_then( | doc | doc.body() )
    .and_then( | body | body.append_child( &web_sys::Element::from( window.canvas() ) ).ok() )
    .expect( "couldn't append canvas to document body" );
    wasm_bindgen_futures::spawn_local( game_template::run( event_loop, window ) );
  }

}

pub fn main()
{
  let renderer = Web::new();
  renderer.run();
}

/* qqq : where is that output from? why?

Compiled shader WebShaderKey(2v3)
game_template_web.js:1184 	Compiled shader WebShaderKey(1v3)
game_template_web.js:1184

*/

/* qqq : output file is too large 10mb
*/

/* qqq : implement release build
*/
