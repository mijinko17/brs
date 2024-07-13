#[derive(Debug)]
pub struct WwnOutput {
    value: [u8; 8],
}

impl WwnOutput {
    pub fn new(value: [u8; 8]) -> Self {
        Self { value }
    }
}
