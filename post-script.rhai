let template_name = variable::get("template_name");
debug(`crate_type: ${template_name}`);

file::rename( `module/renderer/${template_name}/src`, "module/renderer/src" );
file::rename( `module/renderer/${template_name}/Cargo.toml`, "module/renderer/Cargo.toml" );
file::delete( `module/renderer/${template_name}` );