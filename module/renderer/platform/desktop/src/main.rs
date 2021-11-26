use webgpu::
{
  Renderer
};

struct Desktop;

impl Renderer for Desktop
{
  fn new() -> Desktop
  {
    Desktop{}
  }

}

pub fn main()
{
  let renderer = Desktop::new();
  renderer.run();
}

