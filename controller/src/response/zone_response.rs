use serde::Serialize;

#[derive(Serialize)]
pub struct ZoneResponse {
    name: String,
    members: Vec<String>,
}

impl ZoneResponse {
    pub fn new(name: String, members: Vec<String>) -> Self {
        Self { name, members }
    }
}
