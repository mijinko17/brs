use serde::Serialize;
use util::new;

#[derive(Serialize, new)]
pub struct FibrechannelNameServerResponse {
    #[serde(rename = "port-name")]
    pub port_name: String,
}

#[derive(Serialize, new)]
pub struct FibrechannelNameServerWrapResponse {
    #[serde(rename = "fibrechannel-name-server")]
    pub fibrechannel_name_server: Vec<FibrechannelNameServerResponse>,
}
