
use axum::async_trait;

use crate::response::zone_response::ZoneResponse;

#[async_trait]
pub trait ZoneConfigurationController {
    async fn zones(&self) -> Vec<ZoneResponse>;
}
