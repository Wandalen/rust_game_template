let _ = require( 'wTools' );

_.include( 'wFiles' );

let cwd = process.env[ 'CARGO_MAKE_CURRENT_TASK_INITIAL_MAKEFILE_DIRECTORY' ];
let template_name = process.env[ 'GAME_TEMPLATE' ];
let src_src_dir = _.path.join( cwd, template_name, 'src/**' );
let src_cargo_toml = _.path.join( cwd, template_name, 'Cargo.toml' );
let dst_src_dir = _.path.join( cwd, 'src' );
let dst_cargo_toml = _.path.join( cwd, 'Cargo.toml' );

let o = 
{
  reflectMap :
  {
    [ src_src_dir ] : dst_src_dir,
    [ src_cargo_toml ] : dst_cargo_toml
  },
  linkingAction : 'hardLink'
}

_.fileProvider.filesReflect( o );