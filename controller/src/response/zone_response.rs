use serde::Serialize;

#[derive(Serialize)]
pub struct ZoneResponse {
    zone_name: String,
    member_entry: Vec<ZoneMemberEntryResponse>,
}

impl ZoneResponse {
    pub fn new(zone_name: String, members: Vec<ZoneMemberEntryResponse>) -> Self {
        Self {
            zone_name,
            member_entry: members,
        }
    }
}

#[derive(Serialize)]
pub struct ZoneMemberEntryResponse {
    entry_name: Vec<String>,
}
