use util::async_trait;

use crate::{
    input::{create_zone_input::CreateZoneInput, create_zones_input::CreateZonesInput},
    output::{zone_configuration_output::ZoneConfigurationOutput, zone_output::ZoneOutput},
};

#[async_trait]
pub trait ZoneService {
    async fn create_zones(&self, input: CreateZonesInput);
    async fn zones(&self) -> Vec<ZoneOutput>;
    async fn effective_configuration(&self) -> ZoneConfigurationOutput;
}
