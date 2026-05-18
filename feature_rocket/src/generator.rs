use super::args::RocketArgs;
use shared::{
    generator::{BoilerplateGenerator, Feature},
    io::{IoError, IoResult},
    types::{AuthType, DatabaseKind, DatabaseType},
    validation::ValidationError,
};
use std::path::PathBuf;

pub struct RocketGenerator;

impl BoilerplateGenerator for RocketGenerator {
    type Args = RocketArgs;

    fn name(&self) -> &'static str {
        "Rocket"
    }

    fn generate(&self, args: &Self::Args) -> IoResult<()> {
        println!("🚀 Generating Rocket project: {}", args.base.project_name);

        // Create project directory
        let project_directory = args
            .base
            .output_path
            .join(args.base.project_name.to_string());
        std::fs::create_dir_all(&project_directory).map_err(|e| IoError::CreateDir {
            path: project_directory.clone(),
            source: e,
        })?;
        std::process::Command::new("cargo")
            .arg("init")
            .current_dir(&project_directory)
            .status()
            .map_err(|e| IoError::CommandFailed {
                command: "cargo init".to_string(),
                source: e,
            })?;

        // Generate Cargo.toml
        self.generate_cargo_toml(args, &project_directory)?;

        // Generate main.rs
        self.generate_main_rs(args, &project_directory)?;

        // Generate Rocket.toml config
        self.generate_rocket_config(args, &project_directory)?;

        // // Setup database if requested
        // // if let Some(db) = &args.base.databases {

        // // }
        // for db in &args.base.databases {
        //     self.setup_database(db, project_path)?;
        // }

        // // Setup auth if requested
        // if let Some(auth) = &args.base.auth {
        //     self.setup_auth(auth, project_path)?;
        // }

        Ok(())
    }

    fn validate(&self, args: &Self::Args) -> Result<(), ValidationError> {
        // Framework-specific validation
        if args.port == 0 {
            return Err(ValidationError::OutOfRange {
                field: "port".to_string(),
                value: args.port.to_string(),
                min: "1".to_string(),
                max: "65535".to_string(),
            });
        }

        // Validate base args
        args.base.validate()
    }

    fn post_generate(&self, args: &Self::Args, project_path: &PathBuf) -> IoResult<()> {
        println!("📦 Installing dependencies...");

        // Run cargo init
        let project_directory = args.base.output_path.join(project_path);
        let status = std::process::Command::new("cargo")
            .current_dir(project_directory)
            .status()
            .map_err(|e| IoError::CommandFailed {
                command: "cargo build".to_string(),
                source: e,
            })?;

        if !status.success() {
            return Err(IoError::CommandFailed {
                command: "cargo build".to_string(),
                source: std::io::Error::new(std::io::ErrorKind::Other, "Command failed"),
            });
        }

        Ok(())
    }

    fn supported_features(&self) -> Vec<Feature> {
        vec![
            Feature::Database(DatabaseType {
                kind: DatabaseKind::Postgres,
                version: None,
            }),
            Feature::Database(DatabaseType {
                kind: DatabaseKind::MySql,
                version: None,
            }),
            Feature::Database(DatabaseType {
                kind: DatabaseKind::Sqlite,
                version: None,
            }),
            Feature::Auth(AuthType::Jwt),
            Feature::Auth(AuthType::Sessions),
            Feature::Docker,
        ]
    }

    fn required_dependencies(&self) -> Vec<String> {
        vec!["rustc".to_string(), "cargo".to_string()]
    }
}

impl RocketGenerator {
    fn generate_cargo_toml(&self, args: &RocketArgs, path: &PathBuf) -> IoResult<()> {
        let content = format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2024"

[dependencies]
rocket = "0.5.1"
tokio = {{ version = "1", features = ["full"] }}
serde = {{ version = "1", features = ["derive"] }}
"#,
            args.base.project_name
        );

        let cargo_path = path.join("Cargo.toml");
        std::fs::write(&cargo_path, content).map_err(|e| IoError::WriteFile {
            path: cargo_path,
            source: e,
        })?;

        Ok(())
    }

    fn generate_main_rs(&self, _args: &RocketArgs, path: &PathBuf) -> IoResult<()> {
        let content = "
            #[macro_use] extern crate rocket;

            #[get('/<name>/<age>')]
            fn hello(name: &str, age: u8) -> String {
                format!('Hello, {} year old named {}!', age, name)
            }

            #[launch]
            fn rocket() -> _ {
                rocket::build().mount('/hello', routes![hello])
            }
        ";
        let main_rs_path = path.join("src/main.rs");
        std::fs::write(&main_rs_path, content).map_err(|e| IoError::WriteFile {
            path: main_rs_path,
            source: e,
        })?;

        Ok(())
    }
    fn generate_rocket_config(&self, _args: &RocketArgs, path: &PathBuf) -> IoResult<()> {
        let content = format!(
            r#"
            [default]
            address = "0.0.0.0"
"#,
        );

        let rocket_toml_path = path.join("Rocket.toml");
        std::fs::write(&rocket_toml_path, content).map_err(|e| IoError::WriteFile {
            path: rocket_toml_path,
            source: e,
        })?;

        Ok(())
    }
}
