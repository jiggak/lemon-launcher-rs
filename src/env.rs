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

use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::{env, ffi::OsStr, path::{Path, PathBuf}};
use crate::cli::Cli;


fn get_package_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

fn default_config_file_name() -> String {
    String::from("config.toml")
}

fn default_menu_file_name() -> String {
    String::from("menu.toml")
}

fn default_config_dir() -> PathBuf {
    // $XDG_CONFIG_HOME/lemon-launcher, $HOME/.config/lemon-launcher
    let base_data_dir = match env::var("XDG_CONFIG_HOME") {
        Ok(var) => PathBuf::from(var),
        Err(_) => {
            let home_dir = env::var("HOME")
                .expect("HOME env var not found");

            PathBuf::from(home_dir)
                .join(".config")
        }
    };

    base_data_dir.join(get_package_name())
}

fn default_state_dir() -> PathBuf {
    // $XDG_STATE_HOME/lemon-launcher, $HOME/.local/state/lemon-launcher
    let base_data_dir = match env::var("XDG_STATE_HOME") {
        Ok(var) => PathBuf::from(var),
        Err(_) => {
            let home_dir = env::var("HOME")
                .expect("HOME env var not found");

            PathBuf::from(home_dir)
                .join(".local/state")
        }
    };

    base_data_dir.join(get_package_name())
}

#[derive(Deserialize)]
pub struct Env {
    #[serde(rename = "ll_config_file", default = "default_config_file_name")]
    pub config_file_name: String,

    #[serde(rename = "ll_config_home", default = "default_config_dir")]
    pub config_dir: PathBuf,

    #[serde(rename = "ll_menu_file", default = "default_menu_file_name")]
    pub menu_file_name: String,

    #[serde(rename = "ll_menu_home")]
    menu_dir: Option<PathBuf>,

    #[serde(rename = "ll_state_home", default = "default_state_dir")]
    pub state_dir: PathBuf
}

impl Env {
    pub fn load() -> Self {
        // safe to unwrap since all vars have defaults
        serde_env::from_env()
            .expect("env struct members to have defaults")
    }

    pub fn load_from_cli(cli: &Cli) -> Result<Self> {
        if let Some(config_file) = &cli.config {
            let (config_file_name, config_dir) = get_dir_file_pair(config_file.as_ref())?;

            env::set_var("LL_CONFIG_FILE", config_file_name);
            env::set_var("LL_CONFIG_HOME", config_dir);
        }

        if let Some(menu_file) = &cli.menu {
            let (menu_file_name, menu_dir) = get_dir_file_pair(menu_file.as_ref())?;

            env::set_var("LL_MENU_FILE", menu_file_name);
            env::set_var("LL_MENU_HOME", menu_dir);
        }

        Ok(Self::load())
    }

    pub fn get_menu_dir(&self) -> &Path {
        match &self.menu_dir {
            Some(dir) => dir,
            None => &self.config_dir
        }
    }

    /// Get path of file relative to the config dir
    pub fn get_config_file_path(&self, file: impl AsRef<Path>) -> PathBuf {
        self.config_dir.join(file)
    }

    pub fn get_rom_lib_path(&self) -> PathBuf {
        self.state_dir.join("roms.db")
    }

    pub fn get_config_path(&self) -> PathBuf {
        self.config_dir.join(&self.config_file_name)
    }

    pub fn get_menu_path(&self) -> PathBuf {
        self.get_menu_dir().join(&self.menu_file_name)
    }

    pub fn get_keymap_path(&self) -> PathBuf {
        self.state_dir.join("keymap.toml")
    }
}

fn get_dir_file_pair(file_path: &Path) -> Result<(&OsStr, &Path)> {
    let file_name = file_path.file_name()
        .ok_or(anyhow!("Could not get file name from {:?}", file_path))?;
    let parent_dir = file_path.parent()
        .ok_or(anyhow!("Could not get directory from {:?}", file_path))?;
    Ok((file_name, parent_dir))
}
