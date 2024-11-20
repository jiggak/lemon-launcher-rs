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

use std::{collections::VecDeque, path::PathBuf};

use anyhow::Result;
use sdl2::{keyboard::Keycode, pixels::Color, rect::Rect};

use crate::{
    keymap::{Action, ActionToKeycode, Keymap}, lemon_config::Justify,
    lemon_screen::{EventReply, LemonScreen}, renderer::Renderer, MainLoopContext
};

pub struct LemonKeymap {
    file_path: PathBuf,
    actions: VecDeque<Action>,
    keymap: ActionToKeycode
}

impl LemonKeymap {
    pub fn new(file_path: PathBuf) -> Self {
        let actions = vec![
            Action::CursorUp,
            Action::CursorDown,
            Action::PageUp,
            Action::PageDown,
            Action::Select,
            Action::Back,
            Action::Favourite
        ];

        Self {
            file_path,
            actions: actions.into(),
            keymap: ActionToKeycode::new()
        }
    }
}

impl LemonScreen for LemonKeymap {
    fn draw(&self, renderer: &mut Renderer) -> Result<()> {
        renderer.draw_background(Color::BLACK);

        let action = self.actions.front().unwrap();
        let text = format!("Press key for {:?}", action);

        let screen_size = renderer.get_screen_size();
        let screen_rect = Rect::new(0, 0, screen_size.width, screen_size.height);
        let dest = Rect::new(0, 0, screen_size.width, 20)
            .centered_on(screen_rect.center());

        renderer.draw_text(text, Color::WHITE, dest, &Justify::Center)?;

        renderer.present();

        Ok(())
    }

    fn handle_keycode(&mut self, _ctx: &mut MainLoopContext, keycode: &Keycode) -> Result<EventReply> {
        let action = self.actions.pop_front().unwrap();
        self.keymap.insert(action, (*keycode).into());

        if self.actions.is_empty() {
            Keymap::save(&self.keymap, &self.file_path)?;
            println!("Saved keymap to {:?}", self.file_path);

            Ok(EventReply::Exit)
        } else {
            Ok(EventReply::Handled)
        }
    }
}