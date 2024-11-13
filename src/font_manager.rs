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

use anyhow::{Error, Result};

use crate::lemon_config::Font;

pub struct FontManager {
    context: sdl2::ttf::Sdl2TtfContext
}

impl FontManager {
    pub fn init() -> Result<Self> {
        let context = sdl2::ttf::init()?;
        Ok(FontManager {
            context
        })
    }

    pub fn load(&self, font: &Font) -> Result<sdl2::ttf::Font> {
        self.context.load_font(&font.get_font_path(), font.size)
            .map_err(|e| Error::msg(e))
    }
}