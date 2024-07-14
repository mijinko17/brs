use usecase::service::interface::fabric_switch_service::FabricSwitchService;
use util::{async_trait, error_handling::AppResult, new};

use crate::{
    controller::interface::fabric_switch_controller::FabricSwitchController,
    response::{
        fabric_switch_response::{FabricSwitchResponse, FabricSwitchWrapResponse},
        rest_response::RestResponse,
    },
};

#[derive(new)]
pub struct FabricSwitchControllerImpl<T>
where
    T: FabricSwitchService + Sync,
{
    fabric_switch_service: T,
}

#[async_trait]
impl<T> FabricSwitchController for FabricSwitchControllerImpl<T>
where
    T: FabricSwitchService + Sync,
{
    async fn fabric_switches(&self) -> AppResult<RestResponse<FabricSwitchWrapResponse>> {
        let a = self.fabric_switch_service.fabric_switches().await?;
        let b = a.into_iter().map(|fabric_switch_output| {
            FabricSwitchResponse::new(
                fabric_switch_output.ip_address.to_string(),
                fabric_switch_output.firmware_version,
            )
        });
        let c = FabricSwitchWrapResponse::new(b.collect());
        Ok(RestResponse::new(c))
    }
}
