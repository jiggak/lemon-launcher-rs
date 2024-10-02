mod lemon_menu;
mod lemon_launcher;
mod menu_config;

use anyhow::{Error, Result};
use lemon_launcher::LemonLauncher;
use lemon_menu::LemonMenu;
use menu_config::MenuConfig;

fn main() -> Result<()> {
    let sdl_context = sdl2::init()
        .map_err(|e| Error::msg(e))?;

    let ttf_context = sdl2::ttf::init()?;
    let font = ttf_context.load_font("./GamePlayed-vYL7.ttf", 30)
        .map_err(|e| Error::msg(e))?;

    let window = sdl_context.video()
        .map_err(|e| Error::msg(e))?
        .window("Lemon Launcher", 640, 480)
        .position_centered()
        .opengl()
        .build()?;

    sdl_context.mouse()
        .show_cursor(false);

    let config = MenuConfig::load_config("./games.toml")?;
    let mut menu = LemonMenu::new(config);
    let mut app = LemonLauncher::new(font, window)?;

    let mut event_pump = sdl_context.event_pump()
        .map_err(|e| Error::msg(e))?;

    loop {
        let event = event_pump.wait_event();
        if app.handle_event(&event, &mut menu) {
            break;
        }

        app.draw(&menu)?;
    }

    Ok(())
}
