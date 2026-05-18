use super::validation::ValidationError;
use std::path::PathBuf;

use super::types::{AuthType, DatabaseConfig, DatabaseKind, DatabasePurpose, DatabaseType};

// #[derive(Debug, Clone, Default, clap::Args)]
// pub struct BaseArgs {
//     pub project_name: String,
//     pub output_path: PathBuf,
//     pub verbose: bool,
//     pub force: bool,
//     pub databases: Vec<DatabaseConfig>, // ← Multiple databases!
//     pub auth: Option<AuthType>,
// }

#[derive(Debug, Clone, clap::Args)]
pub struct BaseArgs {
    #[arg(short, long)]
    pub project_name: String,

    #[arg(short, long, default_value = ".")]
    pub output_path: PathBuf,

    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    #[arg(short, long)]
    pub force: bool,

    #[arg(long, value_parser = parse_database_config)]
    pub databases: Vec<DatabaseConfig>,

    #[arg(long, value_parser = parse_auth_type)]
    pub auth: Option<AuthType>,
}

// Custom parser for DatabaseConfig
fn parse_database_config(s: &str) -> Result<DatabaseConfig, ValidationError> {
    // Your parsing logic here
    // Example: "postgres:primary" -> DatabaseConfig
    let return_error = Err(ValidationError::InvalidField {
        field: String::from("databases"),
        value: String::from(s),
        expected: String::from("database:usage"),
    });
    let mut parts = s.split(":");
    let kind = match parts.next() {
        Some(k) => match DatabaseKind::from_str(k) {
            Some(kind_obj) => kind_obj,
            None => return return_error,
        },
        None => {
            return return_error;
        }
    };
    let purpose = match parts.next() {
        Some(p) => match DatabasePurpose::from_str(p) {
            Some(purpose_obj) => purpose_obj,
            None => return return_error,
        },
        None => {
            return return_error;
        }
    };
    Ok(DatabaseConfig {
        db_type: DatabaseType {
            kind,
            version: None,
        },
        purpose,
        custom_name: None,
    })
}

// Custom parser for AuthType
fn parse_auth_type(s: &str) -> Result<AuthType, String> {
    match s.to_lowercase().as_str() {
        "jwt" => Ok(AuthType::Jwt),
        "sessions" => Ok(AuthType::Sessions),
        "oauth" => Ok(AuthType::OAuth {}),
        _ => Err(format!("Unknown auth type: {}", s)),
    }
}

impl BaseArgs {
    /// Get primary database (first one marked as Primary, or first in list)
    pub fn primary_database(&self) -> Option<&DatabaseConfig> {
        self.databases
            .iter()
            .find(|db| db.purpose == DatabasePurpose::Primary)
            .or_else(|| self.databases.first())
    }

    /// Get all databases of a specific kind
    pub fn databases_of_kind(&self, kind: DatabaseKind) -> Vec<&DatabaseConfig> {
        self.databases
            .iter()
            .filter(|db| db.db_type.kind == kind)
            .collect()
    }

    /// Check if a specific database type is used
    pub fn has_database(&self, kind: DatabaseKind) -> bool {
        self.databases.iter().any(|db| db.db_type.kind == kind)
    }

    /// Get Redis databases (for cache/sessions)
    pub fn redis_databases(&self) -> Vec<&DatabaseConfig> {
        self.databases
            .iter()
            .filter(|db| db.db_type.kind == DatabaseKind::Redis)
            .collect()
    }

    /// Get SQL databases
    pub fn sql_databases(&self) -> Vec<&DatabaseConfig> {
        self.databases
            .iter()
            .filter(|db| {
                matches!(
                    db.db_type.kind,
                    DatabaseKind::Postgres | DatabaseKind::MySql | DatabaseKind::Sqlite
                )
            })
            .collect()
    }
}

impl BaseArgs {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.project_name.is_empty() {
            return Err(ValidationError::MissingField {
                field: "project_name".to_string(),
            });
        }

        // if !self.force && self.output_path.exists() {
        //     // Check if directory exists and not empty
        //     if self
        //         .output_path
        //         .read_dir()
        //         .ok()
        //         .map(|mut d| d.next().is_some())
        //         .unwrap_or(false)
        //     {
        //         return Err(ValidationError::InvalidField {
        //             field: "output_path".to_string(),
        //             value: self.output_path.display().to_string(),
        //             expected: "empty or non-existent directory".to_string(),
        //         });
        //     }
        // }

        Ok(())
    }
}
