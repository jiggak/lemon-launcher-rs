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

mod cli;
mod env;
mod font_manager;
mod keymap;
mod lemon_config;
mod lemon_keymap;
mod lemon_menu;
mod lemon_screen;
mod lemon_launcher;
mod mame_xml;
mod menu_config;
mod renderer;
mod rom_library;
mod scan;

use anyhow::{Error, Result};
use cli::{Cli, Commands, Parser};

use env::Env;
use keymap::Keymap;
use lemon_config::{LemonConfig, Size};
use lemon_keymap::LemonKeymap;
use lemon_launcher::LemonLauncher;
use lemon_menu::LemonMenu;
use lemon_screen::{EventReply, LemonScreen};
use menu_config::MenuConfig;
use renderer::Renderer;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let env = Env::load_from_cli(&cli)?;

    let config = LemonConfig::load_config(&env.get_config_path())?;

    match cli.command {
        Some(Commands::Scan { mame_xml, genre_ini, roms_dir }) => {
            scan::scan(&mame_xml, &genre_ini, &roms_dir)
        },
        Some(Commands::Keymap { file_path }) => {
            let keymap_path = file_path.unwrap_or_else(|| env.get_keymap_path());

            let ctx = SdlContext::init(&config)?;
            let app = LemonKeymap::new(&config, keymap_path);

            main_loop(ctx, app)
        },
        None | Some(Commands::Launch) => {
            let menu_config = MenuConfig::load_config(&env.get_menu_path())?;
            let menu = LemonMenu::new(menu_config);
            let keymap = Keymap::load(env.get_keymap_path())?;

            let ctx = SdlContext::init(&config)?;
            let app = LemonLauncher::new(config, menu, keymap.into());

            main_loop(ctx, app)
        }
    }
}

struct SdlContext {
    context: sdl2::Sdl,
    renderer: Option<Renderer>,
    screen_size: Size,
    ui_size: Size
}

impl SdlContext {
    fn init(config: &LemonConfig) -> Result<Self> {
        let context = sdl2::init()
            .map_err(|e| Error::msg(e))?;

        context.mouse()
            .show_cursor(false);

        sdl2::image::init(sdl2::image::InitFlag::PNG | sdl2::image::InitFlag::JPG)
            .map_err(|e| Error::msg(e))?;

        let mut sdl = Self {
            context,
            renderer: None,
            screen_size: config.size.clone(),
            ui_size: config.get_ui_size()
        };

        sdl.renderer = Some(sdl.create_renderer()?);

        Ok(sdl)
    }

    fn create_renderer(&mut self) -> Result<Renderer> {
        let window = self.context.video()
            .map_err(|e| Error::msg(e))?
            .window("Lemon Launcher", self.screen_size.width, self.screen_size.height)
            .resizable()
            .position_centered()
            .opengl()
            .build()?;

        Renderer::new(window, &self.ui_size)
    }

    fn close_window(&mut self) {
        self.renderer = None
    }

    fn renderer(&mut self) -> Result<&mut Renderer> {
        if self.renderer.is_none() {
            self.renderer = Some(self.create_renderer()?)
        }

        Ok(self.renderer.as_mut().unwrap())
    }
}

fn main_loop(mut sdl: SdlContext, mut app: impl LemonScreen) -> Result<()> {
    let mut event_pump = sdl.context.event_pump()
        .map_err(|e| Error::msg(e))?;

    loop {
        let event = event_pump.wait_event();
        match app.handle_event(&mut sdl, &event) {
            Ok(EventReply::Exit) => break,
            Err(e) => return Err(e),
            _ => ()
        }

        app.draw(sdl.renderer()?)?;
    }

    Ok(())
}
