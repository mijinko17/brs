use usecase::{
    input::{
        create_zone_input::CreateZoneInput, create_zones_input::CreateZonesInput,
        delete_zone_input::DeleteZoneInput, delete_zones_input::DeleteZonesInput,
        wwn_input::WwnInput,
    },
    service::interface::zone_service::ZoneService,
};
use util::{async_trait, error_handling::AppResult, new, wwn::wwn_from_string};

use crate::{
    controller::interface::zone_controller::ZoneController,
    payload::create_zone_payload::CreateZonePayload,
};

#[derive(new)]
pub struct ZoneControllerImpl<T>
where
    T: ZoneService,
{
    zone_service: T,
}

#[async_trait]
impl<T> ZoneController for ZoneControllerImpl<T>
where
    T: ZoneService + Sync,
{
    async fn create_zone(&self, zone_name: String, payload: CreateZonePayload) -> AppResult<()> {
        let create_zones_input: CreateZonesInput =
            CreateZonesInput::new(vec![CreateZoneInput::new(
                zone_name,
                payload
                    .member_entry
                    .entry_name
                    .into_iter()
                    .map(|wwn_string| wwn_from_string(wwn_string).map(WwnInput::new))
                    .collect::<Result<Vec<_>, _>>()?,
            )]);
        self.zone_service.create_zones(create_zones_input).await
    }

    async fn delete_zone(&self, zone_name: String) -> AppResult<()> {
        self.zone_service
            .remove_zones(DeleteZonesInput::new(vec![DeleteZoneInput(zone_name)]))
            .await
    }
}
