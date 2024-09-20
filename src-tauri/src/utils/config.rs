use std::fs;
use serde::{ Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub dir_root: String,
    pub dir_mods: String,
    pub dir_mymods: String,
    pub dir_mods_raw: String,
    pub path_zipmod_info: String,
    pub path_zipmod_info_raw: String,
}

pub fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let file_path = "src/config.toml";
    let contents = fs::read_to_string(file_path)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

pub fn update_config(name: String, value: String) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "src/config.toml";
    let mut config = read_config().unwrap();
    match name.as_str() {
        "dir_root" => {
            config.dir_root = value;
        }
        "dir_mymods" => {
            config.dir_mymods = value;
        }
        _ => {}
    }

    let config = toml::to_string(&config)?;
    fs::write(file_path, config)?;
    Ok(())
}