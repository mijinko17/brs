use util::new;

use super::zone::Zone;

#[derive(new)]
pub struct ZoneConfiguration {
    pub name: String,
    pub zones: Vec<Zone>,
}
