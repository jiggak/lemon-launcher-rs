mod cli;
mod lemon_config;
mod lemon_menu;
mod lemon_launcher;
mod mame_xml;
mod menu_config;
mod renderer;
mod rom_library;
mod scan;

use anyhow::{Error, Result};
use cli::{Cli, Commands, Parser};

use lemon_config::LemonConfig;
use lemon_launcher::LemonLauncher;
use lemon_menu::LemonMenu;
use menu_config::MenuConfig;
use renderer::Renderer;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = LemonConfig::load_config("./lemon-launcher.toml")?;

    match cli.command {
        Some(Commands::Scan { mame_xml, genre_ini, roms_dir }) => {
            scan::scan(&mame_xml, &genre_ini, &roms_dir)
        },
        None | Some(Commands::Launch) => {
            launch(config)
        }
    }
}

fn launch(config: LemonConfig) -> Result<()> {
    let sdl_context = sdl2::init()
        .map_err(|e| Error::msg(e))?;

    let ttf_context = sdl2::ttf::init()?;
    let _img_context = sdl2::image::init(sdl2::image::InitFlag::PNG | sdl2::image::InitFlag::JPG)
        .map_err(|e| Error::msg(e))?;
    let font = ttf_context.load_font(&config.font_file, config.font_size)
        .map_err(|e| Error::msg(e))?;

    let window = sdl_context.video()
        .map_err(|e| Error::msg(e))?
        .window("Lemon Launcher", 640, 480)
        .position_centered()
        .opengl()
        .build()?;

    sdl_context.mouse()
        .show_cursor(false);

    let mut renderer = Renderer::new(font, window)?;

    let menu_config = MenuConfig::load_config("./games.toml")?;
    let menu = LemonMenu::new(menu_config);
    let mut app = LemonLauncher::new(config, menu);

    let mut event_pump = sdl_context.event_pump()
        .map_err(|e| Error::msg(e))?;

    loop {
        let event = event_pump.wait_event();
        match app.handle_event(&event) {
            // Err(LemonError::Exit) => break,
            Err(e) => return Err(e),
            _ => ()
        }

        app.draw(&mut renderer)?;
    }

    Ok(())
}
