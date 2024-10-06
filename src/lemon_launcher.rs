use anyhow::Result;
use sdl2::{
    event::Event, keyboard::Keycode, rect::Rect
};
use std::path::Path;

use crate::{
    lemon_config::{LemonConfig, LemonMenuConfig}, lemon_menu::LemonMenu,
    renderer::Renderer
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

    pub fn handle_event(&mut self, event: &Event) -> Result<(), EventError> {
        match event {
            Event::Quit { .. } => Err(EventError::Exit),
            Event::KeyDown { keycode: Some(keycode), .. } => {
                match *keycode {
                    Keycode::Up => Ok(self.menu.move_cursor(-1)),
                    Keycode::Down => Ok(self.menu.move_cursor(1)),
                    Keycode::Return => self.menu.activate(),
                    Keycode::Backspace => Ok(self.menu.back()),
                    _ => Ok(())
                }
            }
            _ => Ok(())
        }
    }

    pub fn draw(&self, renderer: &mut Renderer) -> Result<()> {
        // renderer.draw_background(Color::CYAN);
        renderer.draw_image(Path::new(&self.config.background))?;

        draw_menu(
            renderer,
            &self.menu,
            &self.config.menu
        )?;

        renderer.present();

        Ok(())
    }
}

pub enum EventError {
    Exit
}

fn draw_menu(
    renderer: &mut Renderer,
    menu: &LemonMenu,
    options: &LemonMenuConfig
) -> Result<()> {
    let region = options.get_rect();
    let line_height = options.line_height;
    let justify = &options.justify;

    let rows = region.height() / line_height;
    let top_rows = options.hover_offset;
    let bottom_rows = rows - top_rows;

    let mut row_rect = Rect::new(
        region.x,
        region.y + (top_rows * line_height) as i32,
        region.width(),
        line_height
    );

    for entry in menu.iter_fwd().take(bottom_rows as usize) {
        let color = match menu.is_selected(entry) {
            true => options.hover_color,
            false => options.text_color
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

    for entry in menu.iter_rev().take(top_rows as usize) {
        renderer.draw_text(&entry.title, options.text_color, row_rect, justify)?;
        row_rect = row_rect.top_shifted(line_height as i32);
    }

    Ok(())
}
