use crate::entity::zone::Zone;

pub trait ZoneRepository {
    fn save(&self, zones: Vec<Zone>);
    fn zones() -> Vec<Zone>;
}
