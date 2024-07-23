use util::new;

use super::create_zone_input::CreateZoneInput;

#[derive(new)]
pub struct CreateZonesInput {
    pub zone_inputs: Vec<CreateZoneInput>,
}
