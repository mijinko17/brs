use serde::Deserialize;
use util::new;

#[allow(dead_code)]
#[derive(Deserialize, new)]
pub struct CreateZoneConfigurationPayload {
    #[serde(rename = "member-zone")]
    pub member_zone: CreateZoneConfigurationZoneNamePayload,
}

#[allow(dead_code)]
#[derive(Deserialize, new)]
pub struct CreateZoneConfigurationZoneNamePayload {
    #[serde(rename = "zone-name")]
    pub zone_name: Vec<String>,
}
