use util::async_trait;

use crate::entity::fabric_switch::FabricSwitch;

#[async_trait]
pub trait FabricSwitchRespistory {
    async fn fabric_switches(&self) -> Vec<FabricSwitch>;
}
