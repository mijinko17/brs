use util::{async_trait, error_handling::AppResult};

use crate::payload::create_zone_payload::CreateZonePayload;

#[async_trait]
pub trait ZoneController {
    async fn create_zone(&self, zone_name: String, payload: CreateZonePayload) -> AppResult<()>;
    async fn delete_zone(&self, zone_name: String) -> AppResult<()>;
}
