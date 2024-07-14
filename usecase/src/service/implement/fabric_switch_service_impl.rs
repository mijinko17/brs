use util::{async_trait, new};

use crate::{
    output::fabric_switch_output::FabricSwitchOutput,
    repository::fabric_switch_repository::FabricSwitchRespistory,
    service::interface::fabric_switch_service::FabricSwitchService,
};

#[derive(new)]
pub struct FabricSwitchServiceImpl<T>
where
    T: FabricSwitchRespistory,
{
    fabric_switch_repository: T,
}

#[async_trait]
impl<T> FabricSwitchService for FabricSwitchServiceImpl<T>
where
    T: FabricSwitchRespistory + Sync,
{
    async fn fabric_switches(&self) -> Vec<FabricSwitchOutput> {
        self.fabric_switch_repository
            .fabric_switches()
            .await
            .into_iter()
            .map(|fabric_switch| {
                FabricSwitchOutput::new(fabric_switch.ip_address, fabric_switch.firmware_version)
            })
            .collect()
    }
}
