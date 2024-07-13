use super::create_zone_input::CreateZoneInput;

pub struct CreateZonesInput {
    pub zone_inputs: Vec<CreateZoneInput>,
}

impl CreateZonesInput {
    pub fn new(zone_inputs: Vec<CreateZoneInput>) -> Self {
        Self { zone_inputs }
    }
}
