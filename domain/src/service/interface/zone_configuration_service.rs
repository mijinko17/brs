use util::{async_trait, error_handling::AppResult};

use crate::output::zone_configuration_output::ZoneConfigurationOutput;

#[async_trait]
pub trait ZoneConfigurationService {
    async fn effective_configuration(&self) -> AppResult<Option<ZoneConfigurationOutput>>;
}
