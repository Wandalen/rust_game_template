//! ```cargo
//! [dependencies]
//! handlebars = "4.1.4"
//! toml = "0.5.8"
//! serde = { version = "1.0.130", features = ["derive"] }
//! regex = "1.5.4"
//! ```

use regex::Regex;
use serde::{Serializer, Serialize}; /* qqq : ! */

fn env_get_maybe( val: String ) -> String
{
  let re = Regex::new(r"^\$\{(.*)\}").unwrap(); /* qqq : ! */
  if let Some ( caps ) = re.captures( &val )
  {
    if let Some( env_name ) = caps.get( 1 )
    {
      return std::env::var( env_name.as_str() ).expect( &format!( "Env variable {} does't exist.", env_name.as_str() ) )
    }
  }
  val
}

#[derive( Debug, serde::Deserialize )]
#[serde(untagged)]
enum Field<T>
{
  EnvLike( String ),
  Raw( T ),
}

impl <T: std::fmt::Debug + Serialize> serde::Serialize for Field<T>
{
  fn serialize<S>( &self, s: S ) -> Result<S::Ok, S::Error>
  where
      S: Serializer,
  {
    match self
    {
      Field::EnvLike( env_like ) => env_get_maybe( env_like.to_string() ).serialize( s ),
      Field::Raw( raw ) => raw.serialize( s ),
    }
  }
}

#[derive( serde::Serialize, serde::Deserialize )]
struct IOSConfig /* qqq : ! */
{
  development_team: Field<String>, /* qqq : ! */
}

#[derive( serde::Serialize, serde::Deserialize )]
struct Config
{
  ios: IOSConfig, /* qqq : ! */
}

fn main()
{
  let cwd_path = std::env::var( "CARGO_MAKE_CURRENT_TASK_INITIAL_MAKEFILE_DIRECTORY" ).unwrap();
  let cwd = std::path::Path::new( &cwd_path );
  let xcode_project_template = std::fs::read_to_string( cwd.join( "xcode/project.hbs" ) ).unwrap();
  let mobile_template = std::fs::read_to_string( cwd.join( "mobile.hbs" ) ).expect( "Failed to read mobile.hbs" );

  let handlebars = handlebars::Handlebars::new();

  let root_dir_path = cwd.join( "../../../.." );

  let mut private_toml_path = root_dir_path.join( "private.toml" );
  if !private_toml_path.exists()
  {
    eprintln!( "Failed to find private.toml config file. Default configuration file will be used." );
    private_toml_path = root_dir_path.join( "private.toml.hbs" );
  }
  let expect_msg = &format!( "Failed to read {:#?}", private_toml_path );
  let toml_str = std::fs::read_to_string( private_toml_path ).expect( expect_msg );
  let config:Config = toml::from_str( &toml_str ).unwrap(); /* qqq : ! */
  let ios:IOSConfig = config.ios; /* qqq : ! */

  let xcode_project = handlebars.render_template(xcode_project_template.as_str(), &ios ).unwrap();
  std::fs::write( cwd.join( "xcode/project.yml" ), xcode_project ).expect( "Unable to write xcode/project.yml file" );

  let mobile_config = handlebars.render_template(mobile_template.as_str(), &ios ).unwrap();/* qqq : ! */
  std::fs::write( cwd.join( "mobile.toml" ), mobile_config ).expect( "Unable to write mobile.toml file" );
}