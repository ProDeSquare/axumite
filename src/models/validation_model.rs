use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct ValidateUserRequest {
    #[validate(length(
        min = 3,
        max = 20,
        message = "Name must be between 3 and 50 characters"
    ))]
    pub name: String,

    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(range(min = 18, max = 30, message = "Age must be between 18 and 30"))]
    pub age: u8,
}

#[derive(Serialize)]
pub struct ValidatedUserResponse {
    pub success: bool,
    pub message: String,
    pub data: UserData,
}

#[derive(Serialize)]
pub struct UserData {
    pub name: String,
    pub email: String,
    pub age: u8,
}
