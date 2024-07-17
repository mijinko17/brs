use util::new;

use super::delete_zone_input::DeleteZoneInput;

#[derive(new)]
pub struct DeleteZonesInput(pub Vec<DeleteZoneInput>);
