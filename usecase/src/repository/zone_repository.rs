use axum::async_trait;

use crate::entity::zone::Zone;

#[async_trait]
pub trait ZoneRepository {
    async fn save(&self, zones: Vec<Zone>);
    async fn zones(&self) -> Vec<Zone>;
}
