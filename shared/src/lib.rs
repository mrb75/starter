pub mod args;
pub mod generator;
pub mod io;
pub mod types;
pub mod validation;

use std::fmt::Display;

/// Common behavior for all errors in the system
pub trait StarterError: Display + std::fmt::Debug + Send + Sync {
    /// Error code for CLI/API responses
    fn code(&self) -> u32;

    /// User-friendly error message
    fn message(&self) -> String {
        self.to_string()
    }

    /// Should this error be logged?
    fn should_log(&self) -> bool {
        true
    }

    /// Is this error recoverable?
    fn is_recoverable(&self) -> bool {
        false
    }

    /// Helpful hint to fix the error (optional)
    fn hint(&self) -> Option<String> {
        None
    }
}
