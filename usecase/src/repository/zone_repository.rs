use crate::entity::zone::Zone;

pub trait ZoneRepository {
    async fn save(&self, zones: Vec<Zone>);
    async fn zones(&self) -> Vec<Zone>;
}
