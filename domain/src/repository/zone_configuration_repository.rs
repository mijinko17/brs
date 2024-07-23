use util::{async_trait, error_handling::AppResult};

use crate::entity::zone_configuration::ZoneConfiguration;

#[async_trait]
pub trait ZoneConfigurationRepository {
    async fn effective_configuration(&self) -> AppResult<Option<ZoneConfiguration>>;
}
