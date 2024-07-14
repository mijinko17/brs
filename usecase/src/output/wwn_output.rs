#[derive(Debug)]
pub struct WwnOutput {
    pub value: [u8; 8],
}

impl WwnOutput {
    pub fn new(value: [u8; 8]) -> Self {
        Self { value }
    }
}
