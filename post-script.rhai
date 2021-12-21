let template_name = variable::get("template_name");
debug(`crate_type: ${template_name}`);

if template_name == "wgpu" 
{
  file::rename( "module/renderer/wgpu/src", "module/renderer/src" );
  file::rename( "module/renderer/wgpu/Cargo.toml", "module/renderer/Cargo.toml" );
}
else if template_name == "bevy"
{
  file::rename( "module/renderer/bevy/src", "module/renderer/src" );
  file::rename( "module/renderer/bevy/Cargo.toml", "module/renderer/Cargo.toml" );
}