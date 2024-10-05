use anyhow::Result;
use sdl2::rect::Rect;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct LemonConfig {
    pub font_file: String,
    pub font_size: u32,
    pub background: String,
    pub menu: LemonMenuConfig
}

impl LemonConfig {
    pub fn load_config(file_path: &str) -> Result<Self> {
        let toml_src = fs::read_to_string(file_path)?;
        let config:LemonConfig = toml::from_str(&toml_src)?;

        Ok(config)
    }
}

#[derive(Deserialize)]
pub struct LemonMenuConfig {
    pub line_height: u32,
    pub position: Point,
    pub size: Size,
    pub justify: Justify,
    pub text_color: (u8, u8, u8),
    pub hover_color: (u8, u8, u8)
}

impl LemonMenuConfig {
    pub fn get_rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, self.size.width, self.size.height)
    }
}

#[derive(Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

#[derive(Deserialize)]
pub struct Size {
    pub width: u32,
    pub height: u32
}


#[derive(Deserialize)]
pub enum Justify {
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "right")]
    Right
}