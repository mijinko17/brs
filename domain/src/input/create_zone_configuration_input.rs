use util::new;

#[derive(new)]
pub struct CreateZoneConfigurationInput {
    pub config_name: String,
    pub members: Vec<String>,
}
