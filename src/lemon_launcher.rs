use anyhow::Result;
use sdl2::{keyboard::Keycode, rect::Rect};

use crate::{
    env, keymap::{Action, SdlKeycodeToAction},
    lemon_config::{LemonConfig, WidgetContent, WidgetField},
    lemon_menu::LemonMenu,
    lemon_screen::{EventReply, LemonScreen},
    renderer::Renderer
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

    fn handle_action(&mut self, action: &Action) -> Result<EventReply> {
        let row_count = self.config.menu.get_row_count();

        match action {
            Action::CursorUp => self.menu.move_cursor(-1),
            Action::PageUp => self.menu.move_cursor(-row_count),
            Action::CursorDown => self.menu.move_cursor(1),
            Action::PageDown => self.menu.move_cursor(row_count),
            Action::Select => return self.menu.activate(),
            Action::Back => self.menu.back(),
            Action::Favourite => self.menu.toggle_favourite()?
        }

        Ok(EventReply::Handled)
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

    fn draw_widgets(&self, renderer: &mut Renderer) -> Result<()> {
        let item_detail = self.menu.selected_detail();

        for widget in &self.config.widgets {
            match &widget.content {
                WidgetContent::Text { field, template, text_color, justify } => {
                    if let Some(item_detail) = item_detail {
                        let text = match field {
                            WidgetField::Year => item_detail.year.as_ref(),
                            WidgetField::Manufacturer => item_detail.manufacturer.as_ref()
                        };

                        if let Some(text) = text {
                            let text = template.replace("{}", text);
                            renderer.draw_text(text, *text_color, widget.get_rect(), justify)?;
                        }
                    }
                },
                WidgetContent::Favourite { yes_image } => {
                    if let Some(item_detail) = item_detail {
                        let image_path = if item_detail.is_favourite {
                            Some(yes_image)
                        } else {
                            None
                        };

                        if let Some(image_path) = image_path {
                            let image_path = env::get_config_file_path(image_path);
                            renderer.draw_image(&image_path, widget.get_rect())?;
                        }
                    }
                },
                WidgetContent::Image { image } => {
                    let image_path = env::get_config_file_path(image);
                    renderer.draw_image(&image_path, widget.get_rect())?;
                },
                WidgetContent::Screenshot { dir, background, position, size } => {
                    if let Some(screenshot) = self.menu.selected_screenshot() {
                        let screenshot = dir.join(screenshot);
                        if screenshot.exists() {
                            if let Some(background) = background {
                                let background = env::get_config_file_path(background);
                                renderer.draw_image(&background, widget.get_rect())?;
                            }

                            let dest = if let (Some(pos), Some(size)) = (position, size) {
                                Rect::new(pos.x, pos.y, size.width, size.height)
                            } else {
                                widget.get_rect()
                            };

                            renderer.draw_image(&screenshot, dest)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

impl LemonScreen for LemonLauncher {
    fn draw(&self, renderer: &mut Renderer) -> Result<()> {
        self.draw_background(renderer)?;
        self.draw_menu(renderer)?;
        self.draw_widgets(renderer)?;

        renderer.present();

        Ok(())
    }

    fn handle_keycode(&mut self, keycode: &Keycode) -> Result<EventReply> {
        if let Some(action) = self.keymap.get(keycode) {
            // FIXME this clone shouldn't be necessary
            self.handle_action(&action.clone())
        } else {
            Ok(EventReply::Unhandled)
        }
    }
}
