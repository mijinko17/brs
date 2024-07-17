use util::new;

use super::wwn_output::WwnOutput;

#[derive(Debug, new)]
pub struct ZoneOutput {
    pub name: String,
    pub members: Vec<WwnOutput>,
}
