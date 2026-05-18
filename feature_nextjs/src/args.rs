use shared::{
    args::BaseArgs,
    validation::{Valid, ValidationError},
};


#[derive(Debug, Clone)]
pub struct NextJsArgs {
    pub base: BaseArgs, // ← Same base, different extensions

    // Next.js-specific
    pub port: u16,
    pub turbopack: bool,
    pub experimental: bool,
    pub app_dir: bool, // App router vs pages router
}

impl Valid for NextJsArgs {
    fn validate(&self) -> Result<(), ValidationError> {
        self.base.validate()?;

        if self.port == 0 {
            return Err(ValidationError::OutOfRange {
                field: "port".to_string(),
                value: self.port.to_string(),
                min: "1".to_string(),
                max: "65535".to_string(),
            });
        }

        Ok(())
    }
}
