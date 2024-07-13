use super::wwn::Wwn;

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
}
