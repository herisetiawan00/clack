pub fn home_dir() -> Result<String, std::env::VarError> {
    std::env::var("HOME")
}

pub fn config_dir() -> Result<String, std::env::VarError> {
    let home = home_dir()?;

    let dir = std::env::var("XDG_CONFIG_HOME").unwrap_or(format!("{}/.config", home));
    Ok(dir)
}

pub fn app_config_dir() -> Result<String, std::env::VarError> {
    let dir = format!("{}/clack", config_dir()?);
    Ok(dir)
}
