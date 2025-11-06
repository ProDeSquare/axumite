use crate::{
    extractors::validated_json::ValidatedJson,
    models::validation_model::{UserData, ValidateUserRequest, ValidatedUserResponse},
};
use axum::{Json, http::StatusCode};

pub async fn validate_user(
    ValidatedJson(payload): ValidatedJson<ValidateUserRequest>,
) -> (StatusCode, Json<ValidatedUserResponse>) {
    let response = ValidatedUserResponse {
        success: true,
        message: "Validation successful".to_string(),
        data: UserData {
            name: payload.name,
            email: payload.email,
            age: payload.age,
        },
    };

    (StatusCode::OK, Json(response))
}
