use std::slice::Iter;

use anyhow::Result;

use crate::menu_config::{Menu, MenuConfig};

pub struct LemonMenu {
    config: MenuConfig,
    items: Vec<Box<dyn MenuItem>>
}

impl LemonMenu {
    pub fn new(config: MenuConfig) -> Self {
        let items = config.menu.iter()
            .map(|m| Box::new(m.clone()) as Box<dyn MenuItem>)
            .collect();

        LemonMenu {
            config,
            items
        }
    }

    pub fn select(&self) {
    }

    pub fn back(&mut self) {

    }

    pub fn cursor_next(&self, inc: i32) {
        println!("cursor_next {inc}")
    }

    pub fn cursor_prev(&self, inc: i32) {
        println!("cursor_prev {inc}")
    }

    pub fn iter(&self) -> impl Iterator<Item = &Box<dyn MenuItem>> {
        self.items.iter()
    }
}

struct MenuIterator {
    iter: dyn Iterator<Item = Menu>
}

impl Iterator for MenuIterator {
    type Item = Box<dyn MenuItem>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

pub trait MenuItem {
    fn get_title(&self) -> &String;

    fn activate(&self);
}
