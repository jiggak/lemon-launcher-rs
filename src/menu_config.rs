/*
 * Lemon Launcher - SDL based MAME frontend for arcade cabinets
 * Copyright (C) 2024 Josh Kropf <josh@slashdev.ca>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use anyhow::Result;
use serde::Deserialize;
use std::{collections::HashMap, fs, path::{Path, PathBuf}};

use crate::{lemon_launcher::ConfigError, rom_library::Rom};

#[derive(Deserialize)]
pub struct MenuConfig {
    pub main: Menu,
    #[serde(default = "HashMap::new")]
    pub menus: HashMap<String, Menu>
}

impl MenuConfig {
    pub fn load_config(file_path: impl AsRef<Path> + Copy) -> Result<Self, ConfigError> {
        let toml_src = fs::read_to_string(file_path)
            .map_err(|e| ConfigError::io(file_path.as_ref(), e))?;
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
    pub action: MenuEntryAction,
    pub screenshot: Option<PathBuf>,
    pub details: Option<MenuEntryDetail>
}

impl From<&Rom> for MenuEntry {
    fn from(r: &Rom) -> Self {
        let screenshot = PathBuf::from(format!("{}.png", r.name));

        MenuEntry {
            title: r.title.clone(),
            action: MenuEntryAction::Rom {
                rom: r.name.clone(),
                params: None
            },
            screenshot: Some(screenshot),
            details: Some(MenuEntryDetail {
                is_favourite: r.is_favourite,
                year: r.year.clone(),
                manufacturer: r.manufacturer.clone()
            })
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
        exec: String,
        args: Option<Vec<String>>
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

#[derive(Deserialize, Clone, PartialEq)]
pub struct MenuEntryDetail {
    pub is_favourite: bool,
    pub year: Option<String>,
    pub manufacturer: Option<String>
}
