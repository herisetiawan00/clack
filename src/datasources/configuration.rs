use std::fs;
use std::path::PathBuf;

use crate::constants;
use crate::entities::configuration::{Configuration, PartialConfiguration};

pub fn get_configuration() -> Result<Configuration, Box<dyn std::error::Error + Send + Sync>> {
    let config_path = constants::configuration::configuration_path()?;

    let mut user_configuration = PartialConfiguration::empty();

    if PathBuf::from(&config_path).exists() {
        let content = fs::read_to_string(config_path)?;
        user_configuration = toml::from_str(content.as_str())?;
    }

    if user_configuration.is_empty() || user_configuration.with_default_config.unwrap_or(true) {
        let content = include_str!("../../assets/clack.toml");
        let configuration = toml::from_str::<Configuration>(content)?;

        Ok(configuration.merge_with(user_configuration))
    } else {
        Ok(user_configuration.unwrap_all())
    }
}
