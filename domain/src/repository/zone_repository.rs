use util::{async_trait, error_handling::AppResult};

use crate::entity::zone::{Zone, ZoneIdentifier};

#[async_trait]
pub trait ZoneRepository {
    async fn save(&self, zones: Vec<Zone>) -> AppResult<()>;
    async fn delete_by_name(&self, delete_zone_names: Vec<String>) -> AppResult<()>;
    async fn zones(&self) -> AppResult<Vec<(ZoneIdentifier, Zone)>>;
    async fn zones_by_name(&self, names: Vec<String>) -> AppResult<Vec<(ZoneIdentifier, Zone)>>;
}
