use util::async_trait;

use crate::response::{
    fabric_switch_response::FabricSwitchWrapResponse, rest_response::RestResponse,
};

#[async_trait]
pub trait FabricSwitchController {
    async fn fabric_switches(&self) -> RestResponse<FabricSwitchWrapResponse>;
}
