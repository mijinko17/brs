use util::new;

#[derive(new)]
pub struct ConnectedServerOutput {
    wwn: [u8; 8],
}
