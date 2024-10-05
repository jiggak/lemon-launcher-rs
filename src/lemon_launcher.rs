use anyhow::Result;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect
};

use crate::{
    lemon_config::{Justify, LemonConfig}, lemon_menu::LemonMenu, renderer::Renderer
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

    pub fn handle_event(&mut self, event: &Event) -> bool {
        match event {
            Event::Quit { .. } => true,
            Event::KeyDown { keycode: Some(keycode), .. } => {
                match *keycode {
                    Keycode::Up => self.menu.move_cursor(-1),
                    Keycode::Down => self.menu.move_cursor(1),
                    Keycode::Space => self.menu.activate(),
                    Keycode::Backspace => self.menu.back(),
                    _ => ()
                }
                false
            }
            _ => false
        }
    }

    pub fn draw(&self, renderer: &mut Renderer) -> Result<()> {
        renderer.draw_background(Color::CYAN);

        draw_menu(
            &self.menu,
            renderer,
            self.config.menu.get_rect(),
            self.config.menu.line_height,
            &self.config.menu.justify,
            self.config.menu.text_color,
            self.config.menu.hover_color
        )?;

        renderer.present();

        Ok(())
    }
}

fn draw_menu(
    menu: &LemonMenu,
    renderer: &mut Renderer,
    region: Rect,
    line_height: u32,
    justify: &Justify,
    text_color: (u8, u8, u8),
    hover_color: (u8, u8, u8)
) -> Result<()> {
    let rows = region.height() / line_height;
    let top_rows = rows / 2;
    let bottom_rows = rows - top_rows;

    let mut row_rect = Rect::new(
        region.x,
        region.y + (top_rows * line_height) as i32,
        region.width(),
        line_height
    );

    for entry in menu.iter_fwd().take(bottom_rows as usize) {
        let color = match menu.is_selected(entry) {
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

    for entry in menu.iter_rev().take(top_rows as usize) {
        renderer.draw_text(&entry.title, text_color, row_rect, justify)?;
        row_rect = row_rect.top_shifted(line_height as i32);
    }

    Ok(())
}
