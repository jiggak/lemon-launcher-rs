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
use std::{env, ffi::OsStr, path::{Path, PathBuf}};


pub fn init<P: AsRef<Path>>(config_file: Option<P>, menu_file: Option<P>) -> Result<()> {
    // LL_CONFIG_FILE => name of config file
    // LL_CONFIG_HOME => config directory containing config file(s)
    // LL_MENU_PATH => path of menu file
    // LL_STATE_HOME => variable state directory (i.e. roms.db file)

    // TODO Maybe use this to clean this up? https://crates.io/crates/envy
    // I don't like the repetition in init() and the use of unwrap below

    if let Some(config_file) = config_file {
        let (config_file_name, config_dir) = get_dir_file_pair(config_file.as_ref())?;

        env::set_var("LL_CONFIG_FILE", config_file_name);
        env::set_var("LL_CONFIG_HOME", config_dir);
    }

    if let Some(menu_file) = menu_file {
        let (menu_file_name, menu_dir) = get_dir_file_pair(menu_file.as_ref())?;

        env::set_var("LL_MENU_FILE", menu_file_name);
        env::set_var("LL_MENU_HOME", menu_dir);
    }

    set_env_default("LL_CONFIG_FILE", "config.toml");
    set_env_default("LL_CONFIG_HOME", &resolve_config_dir());
    set_env_default("LL_MENU_FILE", "menu.toml");
    set_env_default("LL_MENU_HOME", &get_config_dir());
    set_env_default("LL_STATE_HOME", &resolve_state_dir());

    Ok(())
}

fn get_dir_file_pair(file_path: &Path) -> Result<(&OsStr, &Path)> {
    let file_name = file_path.file_name()
        .ok_or(anyhow!("Could not get file name from {:?}", file_path))?;
    let parent_dir = file_path.parent()
        .ok_or(anyhow!("Could not get directory from {:?}", file_path))?;
    Ok((file_name, parent_dir))
}

fn set_env_default<V: AsRef<OsStr>>(key: &str, value: V) {
    if env::var(key).is_err() {
        env::set_var(key, value);
    }
}

fn resolve_config_dir() -> PathBuf {
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

fn resolve_state_dir() -> PathBuf {
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

fn get_config_dir() -> PathBuf {
    // using unwrap assuming init() is called
    PathBuf::from(env::var("LL_CONFIG_HOME").unwrap())
}

fn get_state_dir() -> PathBuf {
    // using unwrap assuming init() is called
    PathBuf::from(env::var("LL_STATE_HOME").unwrap())
}

fn get_menu_dir() -> PathBuf {
    // using unwrap assuming init() is called
    PathBuf::from(env::var("LL_MENU_HOME").unwrap())
}

/// Get path of file relative to the config dir
pub fn get_config_file_path(file: impl AsRef<Path>) -> PathBuf {
    get_config_dir().join(file)
}

fn get_package_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

pub fn get_rom_lib_path() -> PathBuf {
    get_state_dir().join("roms.db")
}

pub fn get_config_path() -> PathBuf {
    get_config_dir().join(env::var("LL_CONFIG_FILE").unwrap())
}

pub fn get_menu_path() -> PathBuf {
    get_menu_dir().join(env::var("LL_MENU_FILE").unwrap())
}

pub fn get_keymap_path() -> PathBuf {
    get_state_dir().join("keymap.toml")
}