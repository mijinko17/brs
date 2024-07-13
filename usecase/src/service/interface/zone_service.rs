use crate::{
    input::{create_zone_input::CreateZoneInput, create_zones_input::CreateZonesInput},
    output::zone_output::ZoneOutput,
};

pub trait ZoneService {
    async fn create_zones(&self, input: CreateZonesInput);
    async fn zones(&self) -> Vec<ZoneOutput>;
}
