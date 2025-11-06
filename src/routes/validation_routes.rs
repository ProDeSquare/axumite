use axum::{routing::post, Router};

use crate::{controllers::validation_controller, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/validate-user", post(validation_controller::validate_user))
}
