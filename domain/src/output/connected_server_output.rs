use util::new;

#[derive(new)]
pub struct ConnectedServerOutput {
    pub wwn: [u8; 8],
}
