use game_template::
{
  Renderer
};

struct Android;

impl Renderer for Android
{
  fn new() -> Android
  {
    Android{}
  }
}

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
pub fn main()
{
  let renderer = Android::new();
  renderer.run();
}
