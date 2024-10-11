use std::{env, path::{Path, PathBuf}};

pub fn set_screenshots_dir(path: impl AsRef<Path>) {
    env::set_var("LL_SCREENSHOTS_DIR", path.as_ref().as_os_str())
}

pub fn get_screenshots_dir() -> PathBuf {
    match env::var("LL_SCREENSHOTS_DIR") {
        Ok(var) => PathBuf::from(var),
        Err(_) => {
            get_config_dir().join("screenshots")
        }
    }
}

pub fn set_config_dir(path: &str) {
    env::set_var("LL_CONFIG_HOME", path)
}

pub fn get_config_dir() -> PathBuf {
    // get data directory resolve order:
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

fn get_package_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

pub fn get_rom_lib_path() -> PathBuf {
    get_config_dir().join("roms.db")
}

pub fn get_config_path() -> PathBuf {
    get_config_dir().join("lemon-launcher.toml")
}

pub fn get_menu_path() -> PathBuf {
    get_config_dir().join("menu.toml")
}