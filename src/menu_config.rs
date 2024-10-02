use anyhow::Result;
use serde::Deserialize;
use std::{collections::HashMap, fs};

#[derive(Deserialize)]
pub struct MenuConfig {
    pub main:Menu,
    pub menus:HashMap<String, Menu>
}

impl MenuConfig {
    pub fn load_config(file_path: &str) -> Result<Self> {
        let toml_src = fs::read_to_string(file_path)?;
        let config:MenuConfig = toml::from_str(&toml_src)?;

        Ok(config)
    }
}

#[derive(Deserialize, Clone)]
pub struct Menu {
    pub entries:Vec<MenuEntry>
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct MenuEntry {
    pub title:String,
    pub rom:Option<String>,
    pub exec:Option<String>,
    pub menu:Option<String>
}
