use anyhow::Result;
use sdl2::{event::Event, keyboard::Keycode};

use crate::{lemon_launcher::LemonError, renderer::Renderer};

pub trait LemonScreen {
    fn draw(&self, renderer: &mut Renderer) -> Result<()>;

    fn handle_keycode(&mut self, keycode: &Keycode) -> Result<()>;

    fn handle_event(&mut self, event: &Event) -> Result<()> {
        match event {
            Event::Quit { .. } => Err(LemonError::Exit.into()),
            Event::KeyDown { keycode: Some(keycode), .. } => {
                self.handle_keycode(keycode)
            }
            _ => Ok(())
        }
    }
}