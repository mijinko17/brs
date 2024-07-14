use serde::Serialize;
use util::new;

#[derive(Debug, Serialize)]
pub struct ZoneResponse {
    #[serde(rename = "zone-name")]
    pub zone_name: String,
    #[serde(rename = "member-entry")]
    pub member_entry: ZoneMemberEntryResponse,
}

impl ZoneResponse {
    pub fn new(zone_name: String, members: ZoneMemberEntryResponse) -> Self {
        Self {
            zone_name,
            member_entry: members,
        }
    }
}

#[derive(Debug, Serialize, new)]
pub struct ZoneMemberEntryResponse {
    #[serde(rename = "entry-name")]
    pub entry_name: Vec<String>,
}
