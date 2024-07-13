use util::new;

use super::zone::Zone;

#[derive(new)]
pub struct ZoneConfiguration {
    name: String,
    zones: Vec<Zone>,
}
