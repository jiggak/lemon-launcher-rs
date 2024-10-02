use anyhow::Result;
use serde::Deserialize;
use std::fs;

use crate::lemon_menu::MenuItem;

#[derive(Deserialize)]
pub struct MenuConfig {
    pub menu:Vec<Menu>
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
    pub name:String,
    pub entries:Vec<MenuEntry>
}

impl MenuItem for Menu {
    fn get_title(&self) -> &String {
        &self.name
    }

    fn activate(&self) {
    }
}

#[derive(Deserialize, Clone)]
pub struct MenuEntry {
    pub title:String,
    pub rom:Option<String>,
    pub exec:Option<String>
}
