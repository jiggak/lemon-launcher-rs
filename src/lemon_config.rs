use anyhow::Result;
use sdl2::rect::Rect;
use serde::Deserialize;
use std::{fs, path::{Path, PathBuf}};

use crate::env;

#[derive(Deserialize)]
pub struct LemonConfig {
    pub size: Size,
    pub font: Font,
    pub background: Option<Background>,
    pub menu: LemonMenuConfig,
    pub mame: MameCommand,
    pub widgets: Vec<Widget>
}

impl LemonConfig {
    pub fn load_config(file_path: impl AsRef<Path>) -> Result<Self> {
        let toml_src = fs::read_to_string(file_path)?;
        let config:LemonConfig = toml::from_str(&toml_src)?;

        Ok(config)
    }
}

fn default_field_template() -> String {
    String::from("{}")
}

fn default_text_colour() -> (u8, u8, u8) {
    (0xff, 0xff, 0xff)
}

#[derive(Deserialize, Clone)]
pub struct Font {
    pub file: PathBuf,
    pub size: u16
}

impl Font {
    pub fn get_font_path(&self) -> PathBuf {
        env::get_config_file_path(&self.file)
    }
}

#[derive(Deserialize)]
pub struct Background {
    pub image: Option<PathBuf>,
    pub colour: Option<Color>
}

impl Background {
    pub fn get_iamge_path(&self) -> Option<PathBuf> {
        self.image.as_ref().map(|p| env::get_config_file_path(p))
    }
}

#[derive(Deserialize, Clone)]
pub struct MameCommand {
    pub cmd: String,
    pub args: Option<Vec<String>>
}

#[derive(Deserialize)]
pub struct LemonMenuConfig {
    pub hover_offset: u32,
    pub line_height: u32,
    pub position: Point,
    pub size: Size,
    pub justify: Justify,
    pub text_color: Color,
    pub hover_color: Color
}

impl LemonMenuConfig {
    pub fn get_rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, self.size.width, self.size.height)
    }

    pub fn get_row_count(&self) -> i32 {
        self.size.height as i32 / self.line_height as i32
    }
}

pub type Color = (u8, u8, u8);

#[derive(Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

#[derive(Deserialize, Clone)]
pub struct Size {
    pub width: u32,
    pub height: u32
}

impl Size {
    pub fn get_rect(&self) -> Rect {
        Rect::new(0, 0, self.width, self.height)
    }
}

impl From<(u32, u32)> for Size {
    fn from(value: (u32, u32)) -> Self {
        Size { width: value.0, height: value.1 }
    }
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

impl Default for Justify {
    fn default() -> Self { Justify::Left }
}

#[derive(Deserialize)]
pub struct Widget {
    pub position: Point,
    pub size: Size,
    pub content: WidgetContent
}

impl Widget {
    pub fn get_rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, self.size.width, self.size.height)
    }
}

#[derive(Deserialize)]
pub enum WidgetField {
    #[serde(rename = "year")]
    Year,
    #[serde(rename = "manufacturer")]
    Manufacturer
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum WidgetContent {
    #[serde(rename = "text")]
    Text {
        field: WidgetField,
        #[serde(default = "default_field_template")]
        template: String,
        #[serde(default = "default_text_colour")]
        text_color: Color,
        #[serde(default)]
        justify: Justify
    },
    #[serde(rename = "image")]
    Image {
        image: PathBuf
    },
    #[serde(rename = "screenshot")]
    Screenshot {
        dir: PathBuf,
        background: Option<PathBuf>,
        position: Option<Point>,
        size: Option<Size>
    },
    #[serde(rename = "favourite")]
    Favourite {
        yes_image: PathBuf
    }
}
