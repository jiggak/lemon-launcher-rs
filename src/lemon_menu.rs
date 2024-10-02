use crate::menu_config::{Menu, MenuConfig, MenuEntry};

pub struct LemonMenu {
    config: MenuConfig,
    menu: Menu,
    index: usize
}

impl LemonMenu {
    pub fn new(config: MenuConfig) -> Self {
        let menu = config.main.clone();
        LemonMenu {
            config,
            menu,
            index: 0
        }
    }

    pub fn is_selected(&self, entry: &MenuEntry) -> bool {
        &self.menu.entries[self.index] == entry
    }

    pub fn activate(&mut self) {
        let entry = &self.menu.entries[self.index];
        if let Some(menu_id) = &entry.menu {
            self.menu = self.config.menus[menu_id].clone();
            self.index = 0;
        }
    }

    pub fn back(&mut self) {
        self.menu = self.config.main.clone();
        self.index = 0;
    }

    pub fn move_cursor(&mut self, inc: i32) {
        let new_index = self.index as i32 + inc;
        if new_index >= 0 && new_index < self.menu.entries.len() as i32 {
            self.index = new_index as usize;
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &MenuEntry> {
        self.menu.entries.iter()
    }
}
