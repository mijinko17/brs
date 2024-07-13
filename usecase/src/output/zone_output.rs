use super::wwn_output::WwnOutput;

#[derive(Debug)]
pub struct ZoneOutput {
    name: String,
    members: Vec<WwnOutput>,
}

impl ZoneOutput {
    pub fn new(name: String, members: Vec<WwnOutput>) -> Self {
        Self { name, members }
    }
}
