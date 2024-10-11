use std::{env, path::{Path, PathBuf}};

pub fn set_screenshots_dir(path: impl AsRef<Path>) {
    env::set_var("LL_SCREENSHOTS_DIR", path.as_ref().as_os_str())
}

pub fn get_screenshots_dir() -> PathBuf {
    match env::var("LL_SCREENSHOTS_DIR") {
        Ok(var) => PathBuf::from(var),
        Err(_) => {
            get_data_dir().join("screenshots")
        }
    }
}

pub fn get_data_dir() -> PathBuf {
    // get data directory resolve order:
    // $LL_DATA_HOME, $XDG_DATA_HOME/lemon-launcher, $HOME/.local/share/lemon-launcher
    match env::var("LL_DATA_HOME") {
        Ok(var) => PathBuf::from(var),
        Err(_) => {
            let base_data_dir = match env::var("XDG_DATA_HOME") {
                Ok(var) => PathBuf::from(var),
                Err(_) => {
                    let home_dir = env::var("HOME")
                        .expect("HOME env var not found");

                    PathBuf::from(home_dir)
                        .join(".local")
                        .join("share")
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
    get_data_dir().join("roms.db")
}