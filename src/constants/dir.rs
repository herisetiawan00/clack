pub fn home_dir() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let home = std::env::var("HOME")?;
    Ok(home)
}

pub fn config_dir() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let home = home_dir()?;

    let dir = std::env::var("XDG_CONFIG_HOME").unwrap_or(format!("{}/.config", home));
    Ok(dir)
}

pub fn app_config_dir() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let dir = format!("{}/clack", config_dir()?);
    Ok(dir)
}
