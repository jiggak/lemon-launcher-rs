use anyhow::Result;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::{Point, Rect}, render::{TextureQuery, WindowCanvas}, ttf::Font, video::Window
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

    fn draw_text<S: Into<String>>(&mut self, text: S, color: Color, point: Point) -> Result<()> {
        let texture_creator = self.canvas.texture_creator();

        let font_surface = self.font.render(&text.into())
            .solid(color)?;
        let font_texture = texture_creator
            .create_texture_from_surface(&font_surface)?;

        let TextureQuery { width, height, .. } = font_texture.query();

        let font_target = Rect::new(point.x, point.y, width, height);

        self.canvas.copy(&font_texture, None, Some(font_target)).unwrap();

        Ok(())
    }

    fn draw_menu(&mut self, menu: &LemonMenu, region: Rect, line_height: i32) -> Result<()> {
        let mut point = region.top_left();

        // let rows = region.height() / line_height as u32;

        for entry in menu.iter() {
            let color = match menu.is_selected(entry) {
                true => Color::MAGENTA,
                false => Color::BLACK
            };
            self.draw_text(&entry.title, color, point)?;
            point.y += line_height;
        }

        Ok(())
    }
}