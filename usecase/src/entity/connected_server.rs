use util::new;

#[derive(new)]
pub struct ConnectedServer {
    pub wwn: [u8; 8],
}
