use serde::Serialize;
use util::new;

#[derive(Debug, Serialize)]
pub struct ZoneResponse {
    pub zone_name: String,
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
    pub entry_name: Vec<String>,
}
