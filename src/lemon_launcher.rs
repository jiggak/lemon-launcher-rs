use std::path::Path;

use anyhow::Result;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect
};

use crate::{
    lemon_config::LemonConfig, lemon_menu::LemonMenu, renderer::Renderer
};

pub struct LemonLauncher {
    config: LemonConfig,
    menu: LemonMenu
}

impl LemonLauncher {
    pub fn new(config: LemonConfig, menu: LemonMenu) -> Self {
        LemonLauncher {
            config, menu
        }
    }

    pub fn handle_event(&mut self, event: &Event) -> Result<()> {
        let row_count = self.config.get_row_count();
        match event {
            Event::Quit { .. } => Err(LemonError::Exit.into()),
            Event::KeyDown { keycode: Some(keycode), .. } => {
                match *keycode {
                    Keycode::Up => Ok(self.menu.move_cursor(-1)),
                    Keycode::Left => Ok(self.menu.move_cursor(-row_count)),
                    Keycode::Down => Ok(self.menu.move_cursor(1)),
                    Keycode::Right => Ok(self.menu.move_cursor(row_count)),
                    Keycode::Return => self.menu.activate(),
                    Keycode::Backspace => Ok(self.menu.back()),
                    _ => Ok(())
                }
            }
            _ => Ok(())
        }
    }

    pub fn draw(&self, renderer: &mut Renderer) -> Result<()> {
        renderer.draw_background(Color::BLACK);
        renderer.draw_image(Path::new(&self.config.background))?;

        self.draw_menu(renderer)?;

        renderer.present();

        Ok(())
    }

    fn draw_menu(&self, renderer: &mut Renderer) -> Result<()> {
        let region = self.config.menu.get_rect();
        let line_height = self.config.menu.line_height;
        let justify = &self.config.menu.justify;
        let text_color = self.config.menu.text_color;
        let hover_color = self.config.menu.hover_color;

        let rows = region.height() / line_height;
        let top_rows = self.config.menu.hover_offset;
        let bottom_rows = rows - top_rows;

        let mut row_rect = Rect::new(
            region.x,
            region.y + (top_rows * line_height) as i32,
            region.width(),
            line_height
        );

        for entry in self.menu.iter_fwd().take(bottom_rows as usize) {
            let color = match self.menu.is_selected(entry) {
                true => hover_color,
                false => text_color
            };

            renderer.draw_text(&entry.title, color, row_rect, justify)?;
            row_rect = row_rect.bottom_shifted(line_height as i32);
        }

        let mut row_rect = Rect::new(
            region.x,
            region.y + ((top_rows - 1) * line_height) as i32,
            region.width(),
            line_height
        );

        for entry in self.menu.iter_rev().take(top_rows as usize) {
            renderer.draw_text(&entry.title, text_color, row_rect, justify)?;
            row_rect = row_rect.top_shifted(line_height as i32);
        }

        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum LemonError {
    #[error("Exit requested")]
    Exit
}
