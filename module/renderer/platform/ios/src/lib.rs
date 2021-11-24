#[cfg( target_os = "ios" )]

use webgpu::
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





