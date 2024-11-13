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
use sdl2::{event::Event, keyboard::Keycode};

use crate::{renderer::Renderer, SdlContext};

pub trait LemonScreen {
    fn draw(&self, renderer: &mut Renderer) -> Result<()>;

    fn handle_keycode(&mut self, ctx: SdlContext, keycode: &Keycode) -> Result<Option<SdlContext>>;

    fn handle_event(&mut self, ctx: SdlContext, event: &Event) -> Result<Option<SdlContext>> {
        match event {
            Event::Quit { .. } => Ok(None),
            Event::KeyDown { keycode: Some(keycode), .. } => {
                self.handle_keycode(ctx, keycode)
            }
            _ => Ok(Some(ctx))
        }
    }
}
