use serde::Deserialize;
use util::new;

#[allow(dead_code)]
#[derive(Deserialize, new)]
pub struct UpdateZoneConfigurationMemberWrapPayload {
    #[serde(rename = "cfg")]
    config: UpdateZoneConfigurationMemberPayload,
}

#[allow(dead_code)]
#[derive(Deserialize, new)]
pub struct UpdateZoneConfigurationMemberPayload {
    #[serde(rename = "cfg-name")]
    config_name: String,
    #[serde(rename = "member-zone")]
    member_zone: Vec<UpdateZoneConfigurationZoneNamePayload>,
}

#[allow(dead_code)]
#[derive(Deserialize, new)]
pub struct UpdateZoneConfigurationZoneNamePayload {
    #[serde(rename = "zone-name")]
    zone_name: String,
}
