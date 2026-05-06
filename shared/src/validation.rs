use super::StarterError;

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Invalid value '{value}' for field '{field}'. Expected: {expected}")]
    InvalidField {
        field: String,
        value: String,
        expected: String,
    },

    #[error("Missing required field: '{field}'")]
    MissingField { field: String },

    #[error("Invalid value '{value}' for field '{field}'. Expected: value between {min}, {max}")]
    OutOfRange {
        field: String,
        value: String,
        min: String,
        max: String,
    },
    #[error("Field '{requirement}' is required. Expected: {suggestion}")]
    MissingRequirement {
        requirement: String,
        suggestion: String,
    },
}
impl StarterError for ValidationError {
    fn code(&self) -> u32 {
        match self {
            ValidationError::InvalidField { .. } => 4001,
            ValidationError::MissingField { .. } => 4002,
            ValidationError::OutOfRange { .. } => 4003,
            ValidationError::MissingRequirement { .. } => 4004,
        }
    }

    fn message(&self) -> String {
        self.to_string() // Uses thiserror's Display
    }

    fn hint(&self) -> Option<String> {
        match self {
            ValidationError::InvalidField {
                expected, field, ..
            } => Some(format!(
                "Try providing a valid value for '{}' that matches: {}",
                field, expected
            )),
            ValidationError::MissingField { field } => {
                Some(format!("Please provide a value for --{}", field))
            }
            ValidationError::OutOfRange {
                field, min, max, ..
            } => Some(format!(
                "Please provide a value for --{} that be in range of {} and {}",
                field, min, max
            )),
            ValidationError::MissingRequirement {
                requirement,
                suggestion,
            } => Some(format!(
                "Please provide a value for --{} suggestion:{suggestion}",
                requirement
            )),
        }
    }
}

pub trait Valid {
    fn validate(&self) -> Result<(), ValidationError>;
}
