let template_name = variable::get("template_name");
debug(`crate_type: ${template_name}`);

if template_name == "wgpu" 
{
  file::rename( "module/renderer/src/wgpu", "module/renderer/src" );
}
else if template_name == "bevy"
{
  file::rename( "module/renderer/src/bevy", "module/renderer/src" );
}