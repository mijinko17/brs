#[derive(Clone, Copy)]
pub struct Wwn {
    value: [u8; 8],
}

impl Wwn {
    pub fn new(value: [u8; 8]) -> Self {
        Wwn { value }
    }
    pub fn value(&self) -> [u8; 8] {
        self.value.clone()
    }
}
