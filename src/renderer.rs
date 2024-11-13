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

use std::{cmp::min, path::Path};

use anyhow::{Error, Result};
use sdl2::{
    image::LoadTexture, pixels::Color, rect::Rect,
    render::{TextureQuery, WindowCanvas}, video::Window
};
use crate::{font_manager::FontManager, lemon_config::{Font, Justify, Size}};

pub struct Renderer {
    font_manager: FontManager,
    canvas: WindowCanvas
}

impl Renderer {
    pub fn new(window: Window, canvas_size: &Size) -> Result<Self> {
        let mut canvas = window
            .into_canvas()
            .build()?;

        // this allows the interface to scale with the window
        // also allows the UI to be scaled down to arcade monitor res
        canvas.set_logical_size(canvas_size.width, canvas_size.height)?;

        Ok(Renderer {
            font_manager: FontManager::init()?,
            canvas
        })
    }

    pub fn get_screen_size(&self) -> Size {
        self.canvas.logical_size().into()
    }

    pub fn draw_text<S: AsRef<str>, C: Into<Color>>(
        &mut self,
        font: &Font,
        text: S,
        color: C,
        dest: Rect,
        justify: &Justify
    ) -> Result<()> {
        let font = self.font_manager.load(font)?;
        let font_surface = font.render(text.as_ref())
            .blended(color)?;
        let texture_creator = self.canvas.texture_creator();
        let font_texture = texture_creator
            .create_texture_from_surface(&font_surface)?;

        let TextureQuery { width, height, .. } = font_texture.query();

        let width = min(width, dest.width());

        let src_rect = Rect::new(0, 0, width, height);
        let mut dest_rect = Rect::new(dest.x, dest.y, width, height);
        match justify {
            Justify::Center => dest_rect.center_on(dest.center()),
            Justify::Right => dest_rect.offset((dest.width() - width) as i32, 0),
            _ => ()
        };

        self.canvas.copy(&font_texture, Some(src_rect), Some(dest_rect))
            .map_err(|e| Error::msg(e))
    }

    pub fn draw_image(&mut self, img_path: &Path, dest: Rect) -> Result<()> {
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator.load_texture(img_path)
            .map_err(|e| Error::msg(e))?;
        self.canvas.copy(&texture, None, dest)
            .map_err(|e| Error::msg(e))
    }

    pub fn draw_background<C: Into<Color>>(&mut self, color: C) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    pub fn draw_background_image(&mut self, img_path: &Path) -> Result<()> {
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator.load_texture(img_path)
            .map_err(|e| Error::msg(e))?;
        self.canvas.copy(&texture, None, None)
            .map_err(|e| Error::msg(e))
    }

    pub fn present(&mut self) {
        self.canvas.present()
    }
}