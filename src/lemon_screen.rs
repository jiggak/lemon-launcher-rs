use anyhow::Result;
use sdl2::{event::Event, keyboard::Keycode};

use crate::renderer::Renderer;

pub trait LemonScreen {
    fn draw(&self, renderer: &mut Renderer) -> Result<()>;

    fn handle_keycode(&mut self, keycode: &Keycode) -> Result<EventReply>;

    fn handle_event(&mut self, event: &Event) -> Result<EventReply> {
        match event {
            Event::Quit { .. } => Ok(EventReply::Exit),
            Event::KeyDown { keycode: Some(keycode), .. } => {
                self.handle_keycode(keycode)
            }
            _ => Ok(EventReply::Unhandled)
        }
    }
}

pub enum EventReply {
    Handled,
    Unhandled,
    Exit
}