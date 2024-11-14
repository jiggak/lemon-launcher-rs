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
use lemon_config::{Font, LemonConfig, Size};
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

            let app = LemonKeymap::new(keymap_path);

            main_loop(&config, app)
        },
        None | Some(Commands::Launch) => {
            let menu_config = MenuConfig::load_config(&env.get_menu_path())?;
            let menu = LemonMenu::new(menu_config);
            let keymap = Keymap::load(env.get_keymap_path())?;

            let app = LemonLauncher::new(config.clone(), menu, keymap.into());

            main_loop(&config, app)
        }
    }
}

struct MainLoopContext<'ttf> {
    renderer: Option<Renderer<'ttf>>
}

impl<'ttf> MainLoopContext<'ttf> {
    fn close_window(&mut self) {
        self.renderer = None
    }
}

fn new_renderer<'ttf>(
    sdl_context: &sdl2::Sdl,
    ttf_context: &'ttf sdl2::ttf::Sdl2TtfContext,
    screen_size: &Size,
    ui_size: &Size,
    font: &Font
) -> Result<Renderer<'ttf>> {
    let font = ttf_context.load_font(&font.get_font_path(), font.size)
        .map_err(|e| Error::msg(e))?;

    let window = sdl_context.video()
        .map_err(|e| Error::msg(e))?
        .window("Lemon Launcher", screen_size.width, screen_size.height)
        .resizable()
        .position_centered()
        .opengl()
        .build()?;

    Renderer::new(font, window, &ui_size)
}

fn main_loop(config: &LemonConfig, mut app: impl LemonScreen) -> Result<()> {
    let sdl = sdl2::init()
        .map_err(|e| Error::msg(e))?;

    sdl.mouse()
        .show_cursor(false);

    sdl2::image::init(sdl2::image::InitFlag::PNG | sdl2::image::InitFlag::JPG)
        .map_err(|e| Error::msg(e))?;

    let ttf = sdl2::ttf::init()?;

    let mut event_pump = sdl.event_pump()
        .map_err(|e| Error::msg(e))?;

    let mut ctx = MainLoopContext {
        renderer: Some(new_renderer(
            &sdl, &ttf,
            &config.size,
            &config.get_ui_size(),
            &config.font
        )?)
    };

    loop {
        let event = event_pump.wait_event();
        match app.handle_event(&mut ctx, &event) {
            Ok(EventReply::Exit) => break,
            Err(e) => return Err(e),
            _ => ()
        }

        if ctx.renderer.is_none() {
            let renderer = new_renderer(
                &sdl, &ttf,
                &config.size,
                &config.get_ui_size(),
                &config.font
            )?;
            ctx.renderer = Some(renderer);
        }

        app.draw(ctx.renderer.as_mut().unwrap())?;
    }

    Ok(())
}
