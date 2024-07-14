use util::{async_trait, error_handling::AppResult};

use crate::{
    input::create_zones_input::CreateZonesInput,
    output::{zone_configuration_output::ZoneConfigurationOutput, zone_output::ZoneOutput},
};

#[async_trait]
pub trait ZoneService {
    async fn create_zones(&self, input: CreateZonesInput) -> AppResult<()>;
    async fn zones(&self) -> AppResult<Vec<ZoneOutput>>;
    async fn effective_configuration(&self) -> AppResult<ZoneConfigurationOutput>;
}
