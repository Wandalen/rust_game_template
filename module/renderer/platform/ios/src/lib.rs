#[cfg( target_os = "ios" )]

use game_template::Renderer;

struct App;

impl game_template::Renderer for App
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

