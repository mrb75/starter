// use crate::types::DatabaseConfig;

use super::{
    io::{IoError, IoResult},
    types::{AuthType, DatabaseType},
    validation::{Valid, ValidationError},
};
use std::path::PathBuf;

/// Core trait that all boilerplate generators must implement
pub trait BoilerplateGenerator {
    /// The arguments type specific to this framework
    type Args: Valid + Clone + Send + Sync;

    /// Name of the framework (e.g., "Rocket", "Next.js", "Dioxus")
    fn name(&self) -> &'static str;

    /// Generate the boilerplate project
    fn generate(&self, args: &Self::Args) -> IoResult<()>;

    /// Validate arguments before generation (optional, default uses Args::validate)
    fn validate(&self, args: &Self::Args) -> Result<(), ValidationError> {
        args.validate()
    }

    /// Post-generation hook (e.g., install dependencies, git init)
    fn post_generate(&self, _args: &Self::Args, _project_path: &PathBuf) -> IoResult<()> {
        // Default: do nothing
        Ok(())
    }

    /// Clean up if generation fails (optional)
    fn cleanup_on_error(&self, project_path: &PathBuf) -> IoResult<()> {
        // Default: remove the partially created directory
        if project_path.exists() {
            std::fs::remove_dir_all(project_path).map_err(|e| IoError::DeleteDir {
                path: project_path.clone(),
                source: e,
            })?;
        }
        Ok(())
    }

    /// Get supported features for this framework
    fn supported_features(&self) -> Vec<Feature> {
        vec![] // Default: no special features
    }

    /// Get required dependencies for this framework
    fn required_dependencies(&self) -> Vec<String> {
        vec![] // Default: none
    }
}

/// Features that a framework might support
#[derive(Debug, Clone, PartialEq)]
pub enum Feature {
    Database(DatabaseType),
    Auth(AuthType),
    WebAssembly,
    HotReload,
    Docker,
    CiCd,
}
