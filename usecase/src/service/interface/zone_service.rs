use crate::input::{create_zone_input::CreateZoneInput, create_zones_input::CreateZonesInput};

pub trait ZoneService {
    fn create_zones(&self, input: CreateZonesInput);
}
