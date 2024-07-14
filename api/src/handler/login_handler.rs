use axum::http::{HeaderMap, StatusCode};

pub const LOGIN_URL: &str =
    "/rest/login";

pub async fn login_handler() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        "Custom_Basic Tk0ZmY2Zjg3NTY2ZDYwYjhmNj5NGQ0NTkzZjM0M2ZlMWM="
            .parse()
            .unwrap(),
    );
    headers
}
