use serde::Deserialize;
use util::new;

#[allow(dead_code)]
#[derive(Deserialize, new)]
pub struct UpdateZoneConfigurationPayload {
    #[serde(rename = "member-zone")]
    pub member_zone: UpdateZoneConfigurationZoneNamePayload,
}

#[allow(dead_code)]
#[derive(Deserialize, new)]
pub struct UpdateZoneConfigurationZoneNamePayload {
    #[serde(rename = "zone-name")]
    pub zone_name: Vec<String>,
}
