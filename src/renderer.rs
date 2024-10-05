use anyhow::{Error, Result};
use sdl2::{
    pixels::Color, rect::Rect, render::{TextureQuery, WindowCanvas}, ttf::Font,
    video::Window
};
use crate::lemon_config::Justify;

pub struct Renderer<'a, 'b> {
    font: Font<'a, 'b>,
    canvas: WindowCanvas
}

impl<'a, 'b> Renderer<'a, 'b> {
    pub fn new(font: Font<'a, 'b>, window: Window) -> Result<Self> {
        let canvas = window
            .into_canvas()
            .build()?;
        Ok(Renderer {
            font, canvas
        })
    }

    pub fn draw_text<S: Into<String>, C: Into<Color>>(
        &mut self,
        text: S,
        color: C,
        dest: Rect,
        justify: &Justify
    ) -> Result<()> {
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

    pub fn draw_background<C: Into<Color>>(&mut self, color: C) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present()
    }
}