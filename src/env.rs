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

use std::{env, path::{Path, PathBuf}};

pub fn set_config_dir(path: &str) {
    env::set_var("LL_CONFIG_HOME", path)
}

fn get_config_dir() -> PathBuf {
    // get config directory resolve order:
    // $LL_CONFIG_HOME, $XDG_CONFIG_HOME/lemon-launcher, $HOME/.config/lemon-launcher
    match env::var("LL_CONFIG_HOME") {
        Ok(var) => PathBuf::from(var),
        Err(_) => {
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
    }
}

fn get_state_dir() -> PathBuf {
    // get state directory resolve order:
    // $LL_STATE_HOME, $XDG_STATE_HOME/lemon-launcher, $HOME/.local/state/lemon-launcher
    match env::var("LL_STATE_HOME") {
        Ok(var) => PathBuf::from(var),
        Err(_) => {
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
    }
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
    get_config_file_path("lemon-launcher.toml")
}

pub fn get_menu_path() -> PathBuf {
    get_config_file_path("menu.toml")
}

pub fn get_keymap_path() -> PathBuf {
    get_config_file_path("keymap.toml")
}