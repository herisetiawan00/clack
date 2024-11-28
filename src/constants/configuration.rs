use super::dir;

pub fn configuration_path() -> Result<String, Box<dyn std::error::Error>> {
    let dir = format!("{}/clack.toml", dir::app_config_dir()?);
    Ok(dir)
}

pub fn cache_path() -> Result<String, Box<dyn std::error::Error>> {
    let dir = format!("{}/.cache", dir::app_config_dir()?);
    Ok(dir)
}
