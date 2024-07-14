use serde::Serialize;
use util::new;

#[derive(Serialize, new)]
pub struct FabricSwitchResponse {
    #[serde(rename = "ip-address")]
    pub ip_address: String,
    #[serde(rename = "firmware-version")]
    pub firmware_version: String,
}

#[derive(Serialize, new)]
pub struct FabricSwitchWrapResponse {
    #[serde(rename = "fabric-switch")]
    pub fabric_switch: Vec<FabricSwitchResponse>,
}
