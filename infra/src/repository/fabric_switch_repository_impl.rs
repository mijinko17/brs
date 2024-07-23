use domain::{
    entity::fabric_switch::FabricSwitch,
    repository::fabric_switch_repository::FabricSwitchRespistory,
};
use local_ip_address::linux::local_ip;
use std::net::IpAddr;
use util::{async_trait, error_handling::AppResult};

pub struct FabricSwitchRespistoryImpl;

#[async_trait]
impl FabricSwitchRespistory for FabricSwitchRespistoryImpl {
    async fn fabric_switches(&self) -> AppResult<Vec<FabricSwitch>> {
        let my_local_ip = match local_ip().unwrap() {
            IpAddr::V4(ip) => ip,
            IpAddr::V6(_) => panic!("Cannot get local ipv4 address"),
        };
        Ok(vec![FabricSwitch::new(
            my_local_ip,
            "v9.0.0_bld95".to_string(),
        )])
    }
}
