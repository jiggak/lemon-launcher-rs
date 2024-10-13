use anyhow::Result;
use sdl2::{
    event::Event, rect::Rect
};

use crate::{
    keymap::{Action, SdlKeycodeToAction}, lemon_config::LemonConfig,
    lemon_menu::LemonMenu, renderer::Renderer
};

pub struct LemonLauncher {
    config: LemonConfig,
    menu: LemonMenu,
    keymap: SdlKeycodeToAction
}

impl LemonLauncher {
    pub fn new(config: LemonConfig, menu: LemonMenu, keymap: SdlKeycodeToAction) -> Self {
        LemonLauncher {
            config, menu, keymap
        }
    }

    pub fn handle_event(&mut self, event: &Event) -> Result<()> {
        match event {
            Event::Quit { .. } => Err(LemonError::Exit.into()),
            Event::KeyDown { keycode: Some(keycode), .. } => {
                if let Some(action) = self.keymap.get(keycode) {
                    // FIXME this clone shouldn't be necessary
                    self.handle_action(&action.clone())
                } else {
                    Ok(())
                }
            }
            _ => Ok(())
        }
    }

    fn handle_action(&mut self, action: &Action) -> Result<()> {
        let row_count = self.config.menu.get_row_count();

        match action {
            Action::CursorUp => Ok(self.menu.move_cursor(-1)),
            Action::PageUp => Ok(self.menu.move_cursor(-row_count)),
            Action::CursorDown => Ok(self.menu.move_cursor(1)),
            Action::PageDown => Ok(self.menu.move_cursor(row_count)),
            Action::Select => self.menu.activate(),
            Action::Back => Ok(self.menu.back())
        }
    }

    pub fn draw(&self, renderer: &mut Renderer) -> Result<()> {
        self.draw_background(renderer)?;
        self.draw_menu(renderer)?;
        self.draw_screenshot(renderer)?;

        renderer.present();

        Ok(())
    }

    fn draw_background(&self, renderer: &mut Renderer) -> Result<()> {
        if let Some(background) = &self.config.background {
            if let Some(colour) = background.colour {
                renderer.draw_background(colour);
            }

            if let Some(image) = background.get_iamge_path() {
                renderer.draw_image(&image, self.config.size.get_rect())?;
            }
        }

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

    fn draw_screenshot(&self, renderer: &mut Renderer) -> Result<()> {
        if let Some(screenshot) = &self.menu.selected().screenshot {
            if screenshot.exists() {
                let region = self.config.screenshot.get_rect();
                renderer.draw_image(screenshot, region)?;
            } else {
                println!("Screenshot {:?} not found", screenshot);
            }
        }

        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum LemonError {
    #[error("Exit requested")]
    Exit
}
