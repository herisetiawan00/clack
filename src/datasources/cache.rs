use std::{
    fs::{self, File},
    io::Write,
};

use serde::de::DeserializeOwned;

use crate::constants;

fn get_cache_dir() -> Result<String, Box<dyn std::error::Error>> {
    let path = constants::configuration::cache_path()?;
    std::fs::create_dir_all(path.clone())?;
    Ok(path)
}

pub fn store_cache(code: String, value: String) -> Result<(), Box<dyn std::error::Error>> {
    let dir = get_cache_dir()?;
    let path = format!("{}/{}", dir, code);
    let mut file = File::create(&path)?;
    file.write_all(value.as_bytes())?;
    Ok(())
}

pub fn get_cache<T: DeserializeOwned>(code: String) -> Result<T, Box<dyn std::error::Error>> {
    let dir = get_cache_dir()?;
    let path = format!("{}/{}", dir, code);
    let value = fs::read_to_string(path)?;
    let data: T = serde_json::from_str(value.as_str())?;
    Ok(data)
}

pub fn remove_cache(code: String) -> Result<(), Box<dyn std::error::Error>> {
    let dir = get_cache_dir()?;
    let path = format!("{}/{}", dir, code);
    fs::remove_file(path)?;
    Ok(())
}
