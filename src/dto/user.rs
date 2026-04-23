use serde::Deserialize;
use validator::Validate;
use fist_macros::FistRequest;

#[derive(Deserialize, Validate, FistRequest)]
pub struct CreateUserDto {
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(email)]
    pub email: String,
}

