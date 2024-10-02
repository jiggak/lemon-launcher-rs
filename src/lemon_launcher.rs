use anyhow::Result;
use sdl2::{
    event::Event, pixels::Color, rect::{Point, Rect}, render::{TextureQuery, WindowCanvas}, ttf::Font, video::Window
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

    pub fn handle_event(&self, event: &Event) -> bool {
        match event {
            Event::Quit { .. } => true,
            // Event::KeyDown { keycode: Some(keycode), .. } => {
            //     match keycode {
            //         Keycode::W => context.move_up(),
            //         Keycode::A => context.move_left(),
            //         Keycode::S => context.move_down(),
            //         Keycode::D => context.move_right(),
            //         Keycode::Escape => context.toggle_pause(),
            //         _ => false
            //     }
            // }
            _ => false
        }
    }

    pub fn draw(&mut self, menu:&LemonMenu) -> Result<()> {
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

    fn draw_text<S: Into<String>>(&mut self, text: S, point: Point) -> Result<()> {
        let texture_creator = self.canvas.texture_creator();

        let font_surface = self.font.render(&text.into())
            .solid(Color::MAGENTA)?;
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

        for menu in menu.iter() {
            self.draw_text(menu.get_title(), point)?;
            point.y += line_height;
        }

        Ok(())
    }
}