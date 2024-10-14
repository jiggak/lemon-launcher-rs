use std::{collections::VecDeque, path::PathBuf};

use anyhow::Result;
use sdl2::{keyboard::Keycode, pixels::Color, rect::Rect};

use crate::{
    keymap::{Action, ActionToKeycode, Keymap}, lemon_config::Justify,
    lemon_launcher::LemonError, lemon_screen::LemonScreen, renderer::Renderer
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
            Action::Back
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
        let dest = Rect::new(0, 0, screen_size.width, renderer.get_font_height() as u32)
            .centered_on(screen_rect.center());

        renderer.draw_text(text, Color::WHITE, dest, &Justify::Center)?;

        renderer.present();

        Ok(())
    }

    fn handle_keycode(&mut self, keycode: &Keycode) -> Result<()> {
        let action = self.actions.pop_front().unwrap();
        self.keymap.insert(action, (*keycode).into());

        if self.actions.is_empty() {
            Keymap::save(&self.keymap, &self.file_path)?;
            println!("Saved keymap to {:?}", self.file_path);

            Err(LemonError::Exit.into())
        } else {
            Ok(())
        }
    }
}