use util::new;

#[derive(new)]
pub struct ModifyZoneConfigurationMemberInput {
    pub config_name: String,
    pub members: Vec<String>,
}
