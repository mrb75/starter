use std::fmt;

pub trait ValidationError: fmt::Display + std::error::Error + Send + Sync {
    fn code(&self) -> u32;
    fn field(&self) -> Option<&str>;
    fn user_friendly(&self) -> String;
}
pub type ValidationResult<T> = Result<T, Box<dyn ValidationError>>;
pub trait Validation {
    fn validate() -> ValidationResult<()>;
}
