use std::vec;
use util::async_trait;

use crate::{
    entity::{wwn::Wwn, zone::Zone},
    input::create_zones_input::CreateZonesInput,
    output::zone_output::ZoneOutput,
    repository::zone_repository::ZoneRepository,
    service::interface::zone_service::ZoneService,
};

pub struct ZoneServiceImpl<T>
where
    T: ZoneRepository,
{
    repository: T,
}

impl<T> ZoneServiceImpl<T>
where
    T: ZoneRepository,
{
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<T> ZoneService for ZoneServiceImpl<T>
where
    T: ZoneRepository + Sync,
{
    async fn create_zones(&self, input: CreateZonesInput) {
        let zones = input
            .zone_inputs
            .into_iter()
            .map(|zone_input| {
                Zone::new(
                    zone_input.name,
                    zone_input
                        .members
                        .into_iter()
                        .map(|wwn_input| Wwn::new(wwn_input.value))
                        .collect(),
                )
            })
            .collect();
        self.repository.save(zones).await;
    }

    async fn zones(&self) -> Vec<crate::output::zone_output::ZoneOutput> {
        self.repository
            .zones()
            .await
            .into_iter()
            .map(|zone| ZoneOutput::new(zone.name(), vec![]))
            .collect()
    }
}
