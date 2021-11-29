// #[cfg( target_os = "ios" )]

use game_template::
{
  Renderer
};

struct IOS;

impl Renderer for IOS
{
  fn new() -> IOS
  {
    IOS{}
  }

}

#[mobile_entry_point::mobile_entry_point]
pub fn main()
{
  let renderer = IOS::new();
  renderer.run();
}

