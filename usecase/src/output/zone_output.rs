use util::new;

use super::wwn_output::WwnOutput;

#[derive(Debug, new)]
pub struct ZoneOutput {
    pub name: String,
    pub members: Vec<WwnOutput>,
}

// impl ZoneOutput {
//     pub fn new(name: String, members: Vec<WwnOutput>) -> Self {
//         Self { name, members }
//     }
// }
