use shared::{
    generator::{BoilerplateGenerator, Feature},
    io::{IoError, IoResult},
    types::{AuthType, DatabaseKind, DatabaseType},
    validation::{ValidationError},
};
use super::args::NextJsArgs;
use std::path::PathBuf;

pub struct NextJsGenerator;

impl BoilerplateGenerator for NextJsGenerator {
    type Args = NextJsArgs;

    fn name(&self) -> &'static str {
        "Next.js"
    }

    fn generate(&self, args: &Self::Args) -> IoResult<()> {
        println!("📦 Generating Next.js project: {}", args.base.project_name);
        let project_directory = args
            .base
            .output_path
            .join(args.base.project_name.to_string());

        std::fs::create_dir_all(&project_directory).map_err(|e| IoError::CreateDir {
            path: project_directory.clone(),
            source: e,
        })?;

        // Use create-next-app internally
        let status = std::process::Command::new("npx")
            .arg("create-next-app@latest")
            .arg(&args.base.project_name)
            .arg("--ts")
            .arg(if args.turbopack {
                "--turbopack"
            } else {
                "--no-turbopack"
            })
            .current_dir(project_directory.parent().unwrap_or(&PathBuf::from(".")))
            .status()
            .map_err(|e| IoError::CommandFailed {
                command: "create-next-app".to_string(),
                source: e,
            })?;

        if !status.success() {
            return Err(IoError::CommandFailed {
                command: "create-next-app".to_string(),
                source: std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to create Next.js app",
                ),
            });
        }

        Ok(())
    }

    fn validate(&self, args: &Self::Args) -> Result<(), ValidationError> {
        args.base.validate()?;

        // Check if Node.js is installed
        let node_check = std::process::Command::new("node").arg("--version").output();

        if node_check.is_err() {
            return Err(ValidationError::MissingRequirement {
                requirement: "Node.js".to_string(),
                suggestion: "Install Node.js from https://nodejs.org".to_string(),
            });
        }

        Ok(())
    }

    fn required_dependencies(&self) -> Vec<String> {
        vec!["Node.js".to_string(), "npm or yarn".to_string()]
    }

    fn supported_features(&self) -> Vec<Feature> {
        vec![
            Feature::Database(DatabaseType {
                kind: DatabaseKind::Postgres,
                version: None,
            }),
            Feature::Database(DatabaseType {
                kind: DatabaseKind::MongoDb,
                version: None,
            }),
            Feature::Auth(AuthType::OAuth),
            Feature::HotReload,
            Feature::CiCd,
        ]
    }
}
