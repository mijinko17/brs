use axum::http::StatusCode;

pub const LOGOUT_URL: &str = "/rest/logout";

pub async fn logout_handler() -> StatusCode {
    StatusCode::NO_CONTENT
}
