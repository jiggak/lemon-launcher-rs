/*
 * Lemon Launcher - SDL based MAME frontend for arcade cabinets
 * Copyright (C) 2024 Josh Kropf <josh@slashdev.ca>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::{io, path::PathBuf, process::Command};

use anyhow::Result;
use sdl2::{keyboard::Keycode, rect::Rect};

use crate::{
    env::Env,
    keymap::{Action, SdlKeycodeToAction},
    lemon_config::{LemonConfig, MameCommand, ScreenshotWidget, TextWidget, WidgetContent, WidgetField},
    lemon_menu::LemonMenu,
    lemon_screen::{EventReply, LemonScreen},
    menu_config::MenuEntryAction,
    renderer::Renderer,
    rom_library::RomLibrary, SdlContext
};

pub struct LemonLauncher {
    config: LemonConfig,
    menu: LemonMenu,
    keymap: SdlKeycodeToAction,
    env: Env
}

impl LemonLauncher {
    pub fn new(config: LemonConfig, menu: LemonMenu, keymap: SdlKeycodeToAction) -> Self {
        LemonLauncher {
            config, menu, keymap, env: Env::load()
        }
    }

    fn handle_action(&mut self, ctx: &mut SdlContext, action: &Action) -> Result<EventReply> {
        let row_count = self.config.menu.get_row_count();

        match action {
            Action::CursorUp => self.menu.move_cursor(-1),
            Action::PageUp => self.menu.move_cursor(-row_count),
            Action::CursorDown => self.menu.move_cursor(1),
            Action::PageDown => self.menu.move_cursor(row_count),
            Action::Select => return self.handle_select(ctx),
            Action::Back => self.menu.back(),
            Action::Favourite => self.menu.toggle_favourite()?
        }

        Ok(EventReply::Handled)
    }

    fn handle_select(&mut self, ctx: &mut SdlContext) -> Result<EventReply> {
        if let Some(entry) = self.menu.selected() {
            let action = entry.action.clone();
            match action {
                MenuEntryAction::Menu { menu } => {
                    self.menu.open_menu(&menu);
                },
                MenuEntryAction::BuiltIn(_) => {
                    return Ok(EventReply::Exit);
                },
                MenuEntryAction::Query(query) => {
                    self.menu.open_query(&query)?;
                },
                MenuEntryAction::Exec { exec, args } => {
                    exec_command(&exec, args.as_ref())?
                },
                MenuEntryAction::Rom { rom, params } => {
                    let rom_lib = RomLibrary::open()?;
                    rom_lib.inc_play_count(&rom)?;

                    // Close window to let mame use the Linux framebuffer
                    ctx.close_window();

                    self.config.mame.exec(&rom, params.as_ref())?;
                }
            }
        }

        Ok(EventReply::Handled)
    }

    fn draw_background(&self, renderer: &mut Renderer) -> Result<()> {
        if let Some(background) = &self.config.background {
            if let Some(colour) = background.colour {
                renderer.draw_background(colour);
            }

            if let Some(image) = &background.image {
                let image_path = self.env.get_config_file_path(image);
                renderer.draw_background_image(&image_path)?;
            }
        }

        Ok(())
    }

    fn draw_menu(&self, renderer: &mut Renderer) -> Result<()> {
        let region = self.config.menu.get_rect();
        let line_height = self.config.menu.line_height;
        let justify = &self.config.menu.justify;
        let text_color = self.config.menu.text_color;
        let focus_color = self.config.menu.focus_color;

        let rows = region.height() / line_height;
        let top_rows = self.config.menu.focus_offset;
        let bottom_rows = rows - top_rows;

        let mut row_rect = Rect::new(
            region.x,
            region.y + (top_rows * line_height) as i32,
            region.width(),
            line_height
        );

        for entry in self.menu.iter_fwd().take(bottom_rows as usize) {
            let color = match self.menu.is_selected(entry) {
                true => focus_color,
                false => text_color
            };

            renderer.draw_text(&self.config.font, &entry.title, color, row_rect, justify)?;
            row_rect = row_rect.bottom_shifted(line_height as i32);
        }

        let mut row_rect = Rect::new(
            region.x,
            region.y + ((top_rows - 1) * line_height) as i32,
            region.width(),
            line_height
        );

        for entry in self.menu.iter_rev().take(top_rows as usize) {
            renderer.draw_text(&self.config.font, &entry.title, text_color, row_rect, justify)?;
            row_rect = row_rect.top_shifted(line_height as i32);
        }

        Ok(())
    }

    fn draw_widgets(&self, renderer: &mut Renderer) -> Result<()> {
        for widget in &self.config.widgets {
            match &widget.content {
                WidgetContent::Text(text) => {
                    self.draw_text_widget(renderer, widget.get_rect(), text)?;
                },
                WidgetContent::Favourite { yes_image } => {
                    self.draw_favourite_widget(renderer, widget.get_rect(), yes_image)?;
                },
                WidgetContent::Image { image } => {
                    let image_path = self.env.get_config_file_path(image);
                    renderer.draw_image(&image_path, widget.get_rect())?;
                },
                WidgetContent::Screenshot(screenshot) => {
                    self.draw_screenshot_widget(renderer, widget.get_rect(), screenshot)?;
                }
            }
        }

        Ok(())
    }

    fn draw_text_widget(&self,
        renderer: &mut Renderer,
        dest: Rect,
        config: &TextWidget
    ) -> Result<()> {
        if let Some(detail) = self.menu.selected_detail() {
            let text = match config.field {
                WidgetField::Year => detail.year.as_ref(),
                WidgetField::Manufacturer => detail.manufacturer.as_ref()
            };

            if let Some(text) = text {
                let text = config.template.replace("{}", text);
                let text_color = config.text_color
                    .unwrap_or(self.config.menu.text_color);

                renderer.draw_text(&self.config.font, text, text_color, dest, &config.justify)?;
            }
        }

        Ok(())
    }

    fn draw_favourite_widget(&self,
        renderer: &mut Renderer,
        dest: Rect,
        yes_image: &PathBuf
    ) -> Result<()> {
        if let Some(detail) = self.menu.selected_detail() {
            let image_path = if detail.is_favourite {
                Some(yes_image)
            } else {
                None
            };

            if let Some(image_path) = image_path {
                let image_path = self.env.get_config_file_path(image_path);
                renderer.draw_image(&image_path, dest)?;
            }
        }

        Ok(())
    }

    fn draw_screenshot_widget(&self,
        renderer: &mut Renderer,
        dest: Rect,
        config: &ScreenshotWidget
    ) -> Result<()> {
        if let Some(screenshot) = self.menu.selected_screenshot() {
            let screenshot = config.dir.join(screenshot);
            if screenshot.exists() {
                if let Some(background) = &config.background {
                    let background = self.env.get_config_file_path(background);
                    renderer.draw_image(&background, dest)?;
                }

                let dest = if let (Some(pos), Some(size)) = (&config.position, &config.size) {
                    Rect::new(pos.x, pos.y, size.width, size.height)
                } else {
                    dest
                };

                renderer.draw_image(&screenshot, dest)?;
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

    fn handle_keycode(&mut self, ctx: &mut SdlContext, keycode: &Keycode) -> Result<EventReply> {
        if let Some(action) = self.keymap.get(keycode) {
            // FIXME this clone shouldn't be necessary
            self.handle_action(ctx, &action.clone())
        } else {
            Ok(EventReply::Unhandled)
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("Unable to read config file {0}")]
    Io(PathBuf, #[source] std::io::Error),
    #[error("Invalid config file syntax/format")]
    Format(#[from] toml::de::Error)
}

impl ConfigError {
    pub fn io(file_path: &std::path::Path, error: std::io::Error) -> Self {
        ConfigError::Io(file_path.to_path_buf(), error)
    }
}

fn exec_command(cmd: &String, args: Option<&Vec<String>>) -> io::Result<()> {
    let mut cmd = Command::new(cmd);

    if let Some(args) = args {
        cmd.args(args);
    }

    cmd.spawn()?;

    Ok(())
}

impl MameCommand {
    pub fn exec(&self, rom: &String, rom_params: Option<&String>) -> io::Result<()> {
        let mut cmd = Command::new(&self.cmd);

        if let Some(args) = &self.args {
            cmd.args(args);
        }

        if let Some(args) = rom_params {
            cmd.arg(args);
        }

        cmd.arg(rom);

        cmd.spawn()?.wait()?;

        Ok(())
    }
}
