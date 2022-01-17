let _ = require( 'wTools' );

_.include( 'wFiles' );

let cwd = process.env[ 'CARGO_MAKE_CURRENT_TASK_INITIAL_MAKEFILE_DIRECTORY' ];
let template_name = process.env[ 'RUST_GAME_TEMPLATE_PROFILE' ];
let src_src_dir = _.path.join( cwd, template_name, 'src' );
let src_cargo_toml = _.path.join( cwd, template_name, 'Cargo.toml' );
let src_settings_toml = _.path.join( cwd, template_name, 'Extension.toml' );
let dst_src_dir = _.path.join( cwd, 'src' );
let dst_cargo_toml = _.path.join( cwd, 'Cargo.toml' );
let dst_settings_toml = _.path.join( cwd, 'Extension.toml' );

_.fileProvider.filesReflect
({
  reflectMap :
  {
    [ src_cargo_toml ] : dst_cargo_toml,
    [ src_settings_toml ] : dst_settings_toml
  },
  linkingAction : 'hardLink'
});

_.fileProvider.softLink({ srcPath : src_src_dir, dstPath : dst_src_dir });