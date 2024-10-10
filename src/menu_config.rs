use anyhow::Result;
use serde::Deserialize;
use std::{collections::HashMap, fs};

use crate::rom_library::Rom;

#[derive(Deserialize)]
pub struct MenuConfig {
    pub main: Menu,
    pub menus: HashMap<String, Menu>
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
    pub entries: Vec<MenuEntry>
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct MenuEntry {
    pub title: String,
    pub action: MenuEntryAction
}

impl From<&Rom> for MenuEntry {
    fn from(r: &Rom) -> Self {
        MenuEntry {
            title: r.title.clone(),
            action: MenuEntryAction::Rom {
                rom: r.name.clone(),
                params: None
            }
        }
    }
}

#[derive(Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum MenuEntryAction {
    BuiltIn(BuiltInAction),
    /// Open menu
    Menu {
        /// Key of menu to open
        menu: String
    },
    /// Execute shell command
    Exec {
        exec: String
    },
    /// Launch rom using mame
    Rom {
        /// Rom name with file extension (e.g. sf2)
        rom: String,
        /// Optional extra mame arguments
        params: Option<String>
    },
    /// Open menu with entries from rom lib query
    Query(Query)
}

#[derive(Deserialize, Clone, PartialEq)]
pub enum BuiltInAction {
    #[serde(rename="exit")]
    Exit
}

#[derive(Deserialize, Clone, PartialEq)]
#[serde(tag = "query")]
pub enum Query {
    #[serde(rename="categories")]
    Categories,
    #[serde(rename="roms")]
    Roms {
        genre: Option<String>
    },
    #[serde(rename="favourites")]
    Favourites {
        count: u32
    },
    #[serde(rename="popular")]
    Popular {
        count: u32
    }
}
