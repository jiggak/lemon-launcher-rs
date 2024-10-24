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

use anyhow::Result;
use sdl2::rect::Rect;
use serde::Deserialize;
use std::{fs, path::{Path, PathBuf}};

use crate::env;

#[derive(Deserialize)]
pub struct LemonConfig {
    pub size: Size,
    pub ui_size: Option<Size>,
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

    pub fn get_ui_size(&self) -> &Size {
        self.ui_size.as_ref()
            .unwrap_or(&self.size)
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
    Text(TextWidget),
    #[serde(rename = "image")]
    Image {
        image: PathBuf
    },
    #[serde(rename = "screenshot")]
    Screenshot(ScreenshotWidget),
    #[serde(rename = "favourite")]
    Favourite {
        yes_image: PathBuf
    }
}

#[derive(Deserialize)]
pub struct TextWidget {
    pub field: WidgetField,
    #[serde(default = "default_field_template")]
    pub template: String,
    #[serde(default = "default_text_colour")]
    pub text_color: Color,
    #[serde(default)]
    pub justify: Justify
}

#[derive(Deserialize)]
pub struct ScreenshotWidget {
    pub dir: PathBuf,
    pub background: Option<PathBuf>,
    pub position: Option<Point>,
    pub size: Option<Size>
}
