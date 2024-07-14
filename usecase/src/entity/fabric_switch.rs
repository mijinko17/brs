use std::net::Ipv4Addr;

use util::new;

#[derive(new)]
pub struct FabricSwitch {
    pub ip_address: Ipv4Addr,
    pub firmware_version: String,
}
