use shared::{
    args::BaseArgs,
    validation::{Valid, ValidationError},
};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RocketArgs {
    pub base: BaseArgs, // ← Embed common args

    // Rocket-specific
    pub port: u16,
    pub host: String,
    pub workers: Option<usize>,
    pub tls: Option<TlsConfig>,
}

#[derive(Debug, Clone)]
pub struct TlsConfig {
    pub cert_path: PathBuf,
    pub key_path: PathBuf,
}

impl Valid for RocketArgs {
    fn validate(&self) -> Result<(), ValidationError> {
        // Validate base args first
        self.base.validate()?;

        // Validate Rocket-specific args
        if self.port == 0 {
            return Err(ValidationError::OutOfRange {
                field: "port".to_string(),
                value: self.port.to_string(),
                min: "1".to_string(),
                max: "65535".to_string(),
            });
        }

        if self.host.is_empty() {
            return Err(ValidationError::MissingField {
                field: "host".to_string(),
            });
        }

        if let Some(workers) = self.workers {
            if workers == 0 || workers > 32 {
                return Err(ValidationError::OutOfRange {
                    field: "workers".to_string(),
                    value: workers.to_string(),
                    min: "1".to_string(),
                    max: "32".to_string(),
                });
            }
        }

        Ok(())
    }
}
