use util::new;

use super::zone_output::ZoneOutput;

#[derive(new)]
pub struct ZoneConfigurationOutput {
    pub name: String,
    pub zones: Vec<ZoneOutput>,
}
