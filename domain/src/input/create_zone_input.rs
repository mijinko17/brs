use super::wwn_input::WwnInput;

pub struct CreateZoneInput {
    pub name: String,
    pub members: Vec<WwnInput>,
}

impl CreateZoneInput {
    pub fn new(name: String, members: Vec<WwnInput>) -> Self {
        Self { name, members }
    }
}
