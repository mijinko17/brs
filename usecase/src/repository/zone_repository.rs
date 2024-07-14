use util::{async_trait, error_handling::AppResult};

use crate::entity::zone::Zone;

#[async_trait]
pub trait ZoneRepository {
    async fn save(&self, zones: Vec<Zone>) -> AppResult<()>;
    async fn zones(&self) -> AppResult<Vec<Zone>>;
}
