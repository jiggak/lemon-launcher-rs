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

use crate::{renderer::Renderer, MainLoopContext};

pub trait LemonScreen {
    fn draw(&self, renderer: &mut Renderer) -> Result<()>;

    fn handle_keycode(&mut self, ctx: &mut MainLoopContext, keycode: &Keycode) -> Result<EventReply>;

    fn handle_event(&mut self, ctx: &mut MainLoopContext, event: &Event) -> Result<EventReply> {
        match event {
            Event::Quit { .. } => Ok(EventReply::Exit),
            Event::KeyDown { keycode: Some(keycode), .. } => {
                self.handle_keycode(ctx, keycode)
            }
            _ => Ok(EventReply::Unhandled)
        }
    }
}

pub enum EventReply {
    Handled,
    Unhandled,
    Exit
}
