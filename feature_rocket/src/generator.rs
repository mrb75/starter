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

    fn generate(&self, args: &Self::Args, project_path: &PathBuf) -> IoResult<()> {
        println!("🚀 Generating Rocket project: {}", args.base.project_name);

        // Create project directory
        std::fs::create_dir_all(project_path).map_err(|e| IoError::CreateDir {
            path: project_path.clone(),
            source: e,
        })?;

        // Generate Cargo.toml
        self.generate_cargo_toml(args, project_path)?;

        // // Generate main.rs
        // self.generate_main_rs(args, project_path)?;

        // // Generate Rocket.toml config
        // self.generate_rocket_config(args, project_path)?;

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

    fn post_generate(&self, _args: &Self::Args, project_path: &PathBuf) -> IoResult<()> {
        println!("📦 Installing dependencies...");

        // Run cargo init
        let status = std::process::Command::new("cargo")
            .current_dir(project_path)
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
edition = "2021"

[dependencies]
rocket = "0.5.0"
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

    // ... other helper methods
}
