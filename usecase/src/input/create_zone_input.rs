use super::wwn_input::WwnInput;

pub struct CreateZoneInput {
    pub name: String,
    pub members: Vec<WwnInput>,
}
