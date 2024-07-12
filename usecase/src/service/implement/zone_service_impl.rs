use crate::{
    entity::{wwn::Wwn, zone::Zone},
    input::create_zones_input::CreateZonesInput,
    repository::zone_repository::ZoneRepository,
    service::interface::zone_service::ZoneService,
};

pub struct ZoneServiceImpl<T>
where
    T: ZoneRepository,
{
    repository: T,
}

impl<T> ZoneService for ZoneServiceImpl<T>
where
    T: ZoneRepository,
{
    fn create_zones(&self, input: CreateZonesInput) {
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
        self.repository.save(zones);
    }
}
