use util::new;

use super::zone_output::ZoneOutput;

#[derive(new)]
pub struct ZoneConfigurationOutput {
    name: String,
    zones: Vec<ZoneOutput>,
}
