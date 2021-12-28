let template_name = variable::get("template_name");
debug(`crate_type: ${template_name}`);

file::rename( `module/renderer/${template_name}/src`, "module/renderer/src" );
file::rename( `module/renderer/${template_name}/Cargo.toml`, "module/renderer/Cargo.toml" );
file::rename( `module/renderer/${template_name}/Settings.toml`, "module/renderer/Settings.toml" );
file::delete( `module/renderer/${template_name}` );

file::rename( `Readme.md.template`, "Readme.md" );
file::rename( `module/renderer/Readme.md.template`, "module/renderer/Readme.md" );

file::delete( ".github/workflows/AndroidPush.yml" );
file::delete( ".github/workflows/Conditional.yml" );
file::delete( ".github/workflows/DesktopPush.yml" );
file::delete( ".github/workflows/Forward.yml" );
file::delete( ".github/workflows/IosPush.yml" );
file::delete( ".github/workflows/WebPush.yml" );


file::rename( ".github/workflows/AndroidPush.yml.template", ".github/workflows/AndroidPush.yml" );
file::rename( ".github/workflows/Conditional.yml.template", ".github/workflows/Conditional.yml" );
file::rename( ".github/workflows/DesktopPush.yml.template", ".github/workflows/DesktopPush.yml" );
file::rename( ".github/workflows/Forward.yml.template", ".github/workflows/Forward.yml" );
file::rename( ".github/workflows/IosPush.yml.template", ".github/workflows/IosPush.yml" );
file::rename( ".github/workflows/WebPush.yml.template", ".github/workflows/WebPush.yml" );
