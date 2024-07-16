use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateZoneWrapPayload {
    #[serde(rename = "zone")]
    pub zone: CreateZonePayload,
}

#[derive(Deserialize)]
pub struct CreateZonePayload {
    #[serde(rename = "zone-name")]
    pub zone_name: String,
    #[serde(rename = "member-entry")]
    pub member_entry: ZoneMemberEntryNamePayload,
}

#[derive(Deserialize)]
pub struct ZoneMemberEntryNamePayload {
    #[serde(rename = "entry-name")]
    pub entry_name: Vec<String>,
}
