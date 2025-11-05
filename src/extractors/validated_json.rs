use axum::{
    Json,
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use serde_json::json;
use validator::Validate;

pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await.map_err(|err| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": 400,
                    "message": "Invalid JSON",
                    "error": err.to_string(),
                })),
            )
                .into_response()
        })?;

        value.validate().map_err(|errors| {
            let error_map: Vec<_> = errors
                .field_errors()
                .into_iter()
                .map(|(field, errs)| {
                    let messages: Vec<String> = errs
                        .iter()
                        .filter_map(|e| e.message.as_ref().map(|m| m.to_string()))
                        .collect();

                    json!({
                        "field": field,
                        "messages": messages,
                    })
                })
                .collect();

            (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({
                    "status": 422,
                    "message": "Validation failed",
                    "errors": error_map,
                })),
            )
                .into_response()
        })?;

        Ok(ValidatedJson(value))
    }
}
