use anyhow::Result;
use sdl2::rect::Rect;
use serde::Deserialize;
use std::{fs, path::{Path, PathBuf}};

#[derive(Deserialize)]
pub struct LemonConfig {
    pub size: Size,
    pub font_file: String,
    pub font_size: u16,
    pub background: String,
    pub menu: LemonMenuConfig,
    pub screenshot: ScreenshotConfig
}

impl LemonConfig {
    pub fn load_config(file_path: impl AsRef<Path>) -> Result<Self> {
        let toml_src = fs::read_to_string(file_path)?;
        let config:LemonConfig = toml::from_str(&toml_src)?;

        Ok(config)
    }

    pub fn get_row_count(&self) -> i32 {
        self.menu.size.height as i32 / self.menu.line_height as i32
    }

    pub fn get_background_path(&self) -> PathBuf {
        PathBuf::from(&self.background)
    }

    pub fn get_background_rect(&self) -> Rect {
        Rect::new(0, 0, self.size.width, self.size.height)
    }
}

#[derive(Deserialize)]
pub struct LemonMenuConfig {
    pub hover_offset: u32,
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

#[derive(Deserialize)]
pub struct ScreenshotConfig {
    pub dir: Option<PathBuf>,
    pub position: Point,
    pub size: Size
}

impl ScreenshotConfig {
    pub fn get_rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, self.size.width, self.size.height)
    }
}