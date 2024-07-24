use util::new;

use super::zone::Zone;

#[derive(new, Clone)]
pub struct ZoneConfigurationIdentifier(pub u32);

#[derive(new)]
pub struct ZoneConfiguration {
    pub name: String,
    pub zones: Vec<Zone>,
}
