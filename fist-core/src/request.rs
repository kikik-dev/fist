use validator::Validate;
use serde::de::DeserializeOwned;

pub trait FistRequest: DeserializeOwned + Validate {
    fn validate_request(&self) -> Result<(), crate::error::FistError> {
        self.validate().map_err(|e| crate::error::FistError::Validation(e.to_string()))
    }
}
