mod lemon_config;
mod lemon_menu;
mod lemon_launcher;
mod menu_config;
mod renderer;

use anyhow::{Error, Result};
use lemon_config::LemonConfig;
use lemon_launcher::LemonLauncher;
use lemon_menu::LemonMenu;
use menu_config::MenuConfig;
use renderer::Renderer;

fn main() -> Result<()> {
    let config = LemonConfig::load_config("./lemon-launcher.toml")?;

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
        if app.handle_event(&event).is_err() {
            break;
        }

        app.draw(&mut renderer)?;
    }

    Ok(())
}
