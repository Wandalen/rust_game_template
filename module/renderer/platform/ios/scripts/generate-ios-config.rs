//! ```cargo
//! [dependencies]
//! handlebars = "4.1.4"
//! toml = "0.5.8"
//! serde = { version = "1.0.130", features = ["derive"] }
//! ```

#[derive( serde::Serialize, serde::Deserialize )]
struct IOSConfig
{
  development_team: String
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
  let mobile_template = std::fs::read_to_string( cwd.join( "mobile.hbs" ) ).unwrap();

  let handlebars = handlebars::Handlebars::new();

  let toml_str = std::fs::read_to_string( cwd.join( "../../config/private.toml" ) ).unwrap();
  let config:Config = toml::from_str( &toml_str ).unwrap();
  let ios:IOSConfig = config.ios;

  let xcode_project = handlebars.render_template(xcode_project_template.as_str(), &ios ).unwrap();
  std::fs::write( cwd.join( "xcode/project.yml" ), xcode_project ).expect( "Unable to write xcode/project.yml file" );

  let mobile_config = handlebars.render_template(mobile_template.as_str(), &ios ).unwrap();
  std::fs::write( cwd.join( "mobile.toml" ), mobile_config ).expect( "Unable to write mobile.toml file" );
}