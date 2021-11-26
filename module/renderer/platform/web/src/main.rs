#[cfg( target_arch = "wasm32" )]

use webgpu::
{
  common::*,
  Renderer
};

use webgpu::winit::
{
  event_loop::{ EventLoop },
  window::Window,
  platform::web::WindowExtWebSys
};

struct Web;

impl Renderer for Web 
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
    .and_then( |win| win.document() )
    .and_then( |doc| doc.body() )
    .and_then( |body| body.append_child( &web_sys::Element::from( window.canvas() ) ).ok() )
    .expect( "couldn't append canvas to document body" );
    wasm_bindgen_futures::spawn_local( run( event_loop, window ) );
  }
  
}

pub fn main()
{
  let renderer = Web::new();
  renderer.run();
}

