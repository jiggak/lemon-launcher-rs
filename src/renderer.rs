use std::{cmp::min, path::Path};

use anyhow::{Error, Result};
use sdl2::{
    image::LoadTexture, pixels::Color, rect::Rect, render::{TextureQuery, WindowCanvas},
    ttf::Font, video::Window
};
use crate::lemon_config::Justify;

pub struct Renderer<'a, 'b> {
    font: Font<'a, 'b>,
    canvas: WindowCanvas
}

impl<'a, 'b> Renderer<'a, 'b> {
    pub fn new(font: Font<'a, 'b>, window: Window) -> Result<Self> {
        let (win_width, win_height) = window.size();

        let mut canvas = window
            .into_canvas()
            .build()?;

        // using window size as logical size lets the interface scale
        canvas.set_logical_size(win_width, win_height)?;

        Ok(Renderer {
            font, canvas
        })
    }

    pub fn draw_text<S: AsRef<str>, C: Into<Color>>(
        &mut self,
        text: S,
        color: C,
        dest: Rect,
        justify: &Justify
    ) -> Result<()> {
        let font_surface = self.font.render(text.as_ref())
            .blended(color)?;
        let texture_creator = self.canvas.texture_creator();
        let font_texture = texture_creator
            .create_texture_from_surface(&font_surface)?;

        let TextureQuery { width, height, .. } = font_texture.query();

        let width = min(width, dest.width());

        let src_rect = Rect::new(0, 0, width, height);
        let mut dest_rect = Rect::new(dest.x, dest.y, width, height);
        match justify {
            Justify::Center => dest_rect.center_on(dest.center()),
            Justify::Right => dest_rect.offset((dest.width() - width) as i32, 0),
            _ => ()
        };

        self.canvas.copy(&font_texture, Some(src_rect), Some(dest_rect))
            .map_err(|e| Error::msg(e))
    }

    pub fn draw_image(&mut self, img_path: &Path, dest: Rect) -> Result<()> {
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator.load_texture(img_path)
            .map_err(|e| Error::msg(e))?;
        self.canvas.copy(&texture, None, dest)
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