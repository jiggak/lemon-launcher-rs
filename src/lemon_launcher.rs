use anyhow::{Error, Result};
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::{TextureQuery, WindowCanvas}, ttf::Font, video::Window
};

use crate::lemon_menu::LemonMenu;

pub struct LemonLauncher<'a, 'b> {
    font: Font<'a, 'b>,
    canvas: WindowCanvas
}

impl<'a, 'b> LemonLauncher<'a, 'b> {
    pub fn new(font: Font<'a, 'b>, window: Window) -> Result<Self> {
        let canvas = window
            .into_canvas()
            .build()?;
        Ok(LemonLauncher {
            font,
            canvas: canvas
        })
    }

    pub fn handle_event(&self, event: &Event, menu: &mut LemonMenu) -> bool {
        match event {
            Event::Quit { .. } => true,
            Event::KeyDown { keycode: Some(keycode), .. } => {
                match *keycode {
                    Keycode::Up => menu.move_cursor(-1),
                    Keycode::Down => menu.move_cursor(1),
                    Keycode::Space => menu.activate(),
                    Keycode::Backspace => menu.back(),
                    _ => ()
                }
                false
            }
            _ => false
        }
    }

    pub fn draw(&mut self, menu: &LemonMenu) -> Result<()> {
        self.draw_background();
        // self.draw_text("Hello, World!", Point::new(50, 50))?;
        self.draw_menu(menu, Rect::new(40, 40, 560, 400), 40)?;

        self.canvas.present();

        Ok(())
    }

    fn draw_background(&mut self) {
        self.canvas.set_draw_color(Color::CYAN);
        self.canvas.clear();
    }

    fn draw_text<S: Into<String>>(&mut self, text: S, color: Color, dest: Rect, justify: Justify) -> Result<()> {
        let texture_creator = self.canvas.texture_creator();

        let font_surface = self.font.render(&text.into())
            .blended(color)?;
        let font_texture = texture_creator
            .create_texture_from_surface(&font_surface)?;

        let TextureQuery { width, height, .. } = font_texture.query();

        let mut font_rect = Rect::new(dest.x, dest.y, width, height);
        match justify {
            Justify::Center => font_rect.center_on(dest.center()),
            Justify::Right => font_rect.offset((dest.width() - width) as i32, 0),
            _ => ()
        };

        self.canvas.copy(&font_texture, None, Some(font_rect))
            .map_err(|e| Error::msg(e))
    }

    fn draw_menu(&mut self, menu: &LemonMenu, region: Rect, line_height: u32) -> Result<()> {
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
                true => Color::MAGENTA,
                false => Color::BLACK
            };

            self.draw_text(&entry.title, color, row_rect, Justify::Center)?;
            row_rect = row_rect.bottom_shifted(line_height as i32);
        }

        let mut row_rect = Rect::new(
            region.x,
            region.y + ((top_rows - 1) * line_height) as i32,
            region.width(),
            line_height
        );

        for entry in menu.iter_rev().take(top_rows as usize) {
            self.draw_text(&entry.title, Color::BLACK, row_rect, Justify::Center)?;
            row_rect = row_rect.top_shifted(line_height as i32);
        }

        Ok(())
    }
}

pub enum Justify {
    Left,
    Center,
    Right
}