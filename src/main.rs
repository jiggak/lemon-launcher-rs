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

use std::path::PathBuf;

use anyhow::{Error, Result};
use cli::{Cli, Commands, Parser};

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

    env::init(cli.config.as_ref(), cli.menu.as_ref())?;

    let config = LemonConfig::load_config(&env::get_config_path())?;

    match cli.command {
        Some(Commands::Scan { mame_xml, genre_ini, roms_dir }) => {
            scan::scan(&mame_xml, &genre_ini, &roms_dir)
        },
        Some(Commands::Keymap { file_path }) => {
            launch_keymap(config, file_path.unwrap_or_else(|| env::get_keymap_path()))
        },
        None | Some(Commands::Launch) => {
            launch(config)
        }
    }
}

fn launch(config: LemonConfig) -> Result<()> {
    let menu_config = MenuConfig::load_config(&env::get_menu_path())?;
    let menu = LemonMenu::new(menu_config, config.mame.clone());
    let keymap = Keymap::load(env::get_keymap_path())?;

    let screen_size = config.size.clone();
    let ui_size = config.get_ui_size().clone();
    let font = config.font.clone();
    let app = LemonLauncher::new(config, menu, keymap.into());

    launch_ui(&screen_size, &ui_size, &font, app)
}

fn launch_keymap(config: LemonConfig, file_path: PathBuf) -> Result<()> {
    let app = LemonKeymap::new(file_path);

    launch_ui(&config.size, config.get_ui_size(), &config.font, app)
}

fn launch_ui(screen_size: &Size, ui_size: &Size, font: &Font, mut app: impl LemonScreen) -> Result<()> {
    let sdl_context = sdl2::init()
        .map_err(|e| Error::msg(e))?;

    let _img_context = sdl2::image::init(sdl2::image::InitFlag::PNG | sdl2::image::InitFlag::JPG)
        .map_err(|e| Error::msg(e))?;

    let ttf_context = sdl2::ttf::init()?;
    // let font_bytes = sdl2::rwops::RWops::from_bytes(include_bytes!("../config/PressStart2P-vaV7.ttf"))
    //     .map_err(|e| Error::msg(e))?;
    // let font = ttf_context.load_font_from_rwops(font_bytes, font.size)
    let font = ttf_context.load_font(&font.get_font_path(), font.size)
        .map_err(|e| Error::msg(e))?;

    let window = sdl_context.video()
        .map_err(|e| Error::msg(e))?
        .window("Lemon Launcher", screen_size.width, screen_size.height)
        .resizable()
        .position_centered()
        .opengl()
        .build()?;

    sdl_context.mouse()
        .show_cursor(false);

    let mut renderer = Renderer::new(font, window, ui_size)?;

    let mut event_pump = sdl_context.event_pump()
        .map_err(|e| Error::msg(e))?;

    loop {
        let event = event_pump.wait_event();
        match app.handle_event(&event) {
            Ok(EventReply::Exit) => break,
            Err(e) => return Err(e),
            _ => ()
        }

        app.draw(&mut renderer)?;
    }

    Ok(())
}
