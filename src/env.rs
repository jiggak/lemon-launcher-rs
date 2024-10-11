use std::{env, path::{Path, PathBuf}};

pub fn set_screenshots_dir(path: impl AsRef<Path>) {
    env::set_var("LL_SCREENSHOTS_DIR", path.as_ref().as_os_str())
}

pub fn get_screenshots_dir() -> PathBuf {
    match env::var("LL_SCREENSHOTS_DIR") {
        Ok(var) => PathBuf::from(var),
        Err(_) => {
            PathBuf::from("screenshots")
        }
    }
}