use crate::handle::{login_verify1, login_verify2, login_verify3};
use axum::Router;
use axum::routing::post;

pub fn creatroute() -> Router {
    Router::new()
        .route("/verify/submit_ticket", post(login_verify1))
        .route("/verify/submit_phone_number", post(login_verify2))
        .route("/verify/submit_auth", post(login_verify3))
}
