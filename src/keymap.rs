use std::{collections::HashMap, fs, path::Path};

use anyhow::Result;
use sdl2::keyboard::Keycode as SdlKeycode;
use serde::{Deserialize, Serialize};

pub struct Keymap {
    keymap: ActionToKeycode
}

pub type ActionToKeycode = HashMap<Action, Keycode>;
pub type SdlKeycodeToAction = HashMap<SdlKeycode, Action>;

impl Keymap {
    pub fn load(file_path: impl AsRef<Path>) -> Result<Keymap> {
        if let Ok(toml_src) = fs::read_to_string(file_path) {
            let keymap = toml::from_str(&toml_src)?;
            Ok(Keymap { keymap })
        } else {
            Ok(Keymap::default())
        }
    }

    pub fn save(keymap: &ActionToKeycode, file_path: impl AsRef<Path>) -> Result<()> {
        let toml_src = toml::to_string(keymap)?;
        Ok(fs::write(file_path, toml_src)?)
    }
}

impl Default for Keymap {
    fn default() -> Self {
        Self {
            keymap: HashMap::from([
                (Action::CursorUp, SdlKeycode::Up.into()),
                (Action::CursorDown, SdlKeycode::Down.into()),
                (Action::PageUp, SdlKeycode::Left.into()),
                (Action::PageDown, SdlKeycode::Right.into()),
                (Action::Select, SdlKeycode::Return.into()),
                (Action::Back, SdlKeycode::Backspace.into())
            ])
        }
    }
}

impl From<Keymap> for SdlKeycodeToAction {
    fn from(keymap: Keymap) -> Self {
        keymap.keymap.into_iter()
            .flat_map(|(action, keycode)| {
                match keycode {
                    Keycode::Single(k) => vec![(
                        SdlKeycode::from_i32(k).unwrap(),
                        action
                    )],
                    Keycode::Many(keys) => keys.into_iter()
                        .map(|k| (
                            SdlKeycode::from_i32(k).unwrap(),
                            action.clone()
                        ))
                        .collect()
                }
            })
            .collect()
    }
}

// Keycodes are integers defined in SDL_KeyCode
// https://github.com/Rust-SDL2/rust-sdl2/blob/master/sdl2-sys/sdl_bindings.rs#L7659
#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum Keycode {
    Single(i32),
    Many(Vec<i32>)
}

impl From<SdlKeycode> for Keycode {
    fn from(value: SdlKeycode) -> Self {
        Self::Single(*value)
    }
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Hash, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    CursorUp,
    CursorDown,
    PageUp,
    PageDown,
    Select,
    Back
}