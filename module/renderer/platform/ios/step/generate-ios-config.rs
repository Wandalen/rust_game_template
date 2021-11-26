//! ```cargo
//! [dependencies]
//! handlebars = "4.1.4"
//! toml = "0.5.8"
//! serde = { version = "1.0.130", features = ["derive"] }
//! regex = "1.5.4"
//! ```

use regex::Regex;
use serde::de::{Deserialize, Deserializer};
use serde::{Serializer, Serialize};

fn env_get_maybe( val: String ) -> String
{
  let re = Regex::new(r"^\$\{(.*)\}").unwrap();
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
struct IOSConfig
{
  development_team: Field<String>,
}

#[derive( serde::Serialize, serde::Deserialize )]
struct Config
{
  ios: IOSConfig,
}

fn main()
{
  let cwd_path = std::env::var( "CARGO_MAKE_CURRENT_TASK_INITIAL_MAKEFILE_DIRECTORY" ).unwrap();
  let cwd = std::path::Path::new( &cwd_path );
  let xcode_project_template = std::fs::read_to_string( cwd.join( "xcode/project.hbs" ) ).unwrap();
  let mobile_template = std::fs::read_to_string( cwd.join( "mobile.hbs" ) ).expect( "Failed to read mobile.hbs" );

  let handlebars = handlebars::Handlebars::new();

  let toml_str = std::fs::read_to_string( cwd.join( "../../../../private.toml" ) ).expect( "Failed to read config/private.toml" );
  let config:Config = toml::from_str( &toml_str ).unwrap();
  let ios:IOSConfig = config.ios;

  let xcode_project = handlebars.render_template(xcode_project_template.as_str(), &ios ).unwrap();
  std::fs::write( cwd.join( "xcode/project.yml" ), xcode_project ).expect( "Unable to write xcode/project.yml file" );

  let mobile_config = handlebars.render_template(mobile_template.as_str(), &ios ).unwrap();
  std::fs::write( cwd.join( "mobile.toml" ), mobile_config ).expect( "Unable to write mobile.toml file" );
}