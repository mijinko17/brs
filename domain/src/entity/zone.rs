use util::new;

use super::wwn::Wwn;

#[derive(new)]
pub struct ZoneIdentifier(pub u32);

#[derive(Clone)]
pub struct Zone {
    name: String,
    members: Vec<Wwn>,
}

impl Zone {
    pub fn new(name: String, members: Vec<Wwn>) -> Self {
        Zone { name, members }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn members(&self) -> Vec<Wwn> {
        self.members.clone()
    }
}
