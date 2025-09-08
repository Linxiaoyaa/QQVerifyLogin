use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct APIResult<T: Serialize> {
    pub code: i64,
    pub reqststus: String,
    pub data: T,
}

impl<T: Serialize> IntoResponse for APIResult<T> {
    fn into_response(self) -> Response {
        let jsonbody = Json(self);
        jsonbody.into_response()
    }
}
