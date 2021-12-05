#[cfg( target_os = "ios" )]

mod common;

use common::Renderer;

struct App;

impl common::Renderer for App
{
  fn new() -> App
  {
    App{}
  }
}

#[mobile_entry_point::mobile_entry_point]
pub fn main()
{
  let app = App::new();
  app.run();
}

