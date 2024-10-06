use crate::{lemon_launcher::EventError, menu_config::{BuiltInAction, Menu, MenuConfig, MenuEntry, MenuEntryAction}};

pub struct LemonMenu {
    config: MenuConfig,
    menu: Menu,
    index: usize,
    history: Vec<(Menu, usize)>
}

impl LemonMenu {
    pub fn new(config: MenuConfig) -> Self {
        let menu = config.main.clone();
        LemonMenu {
            config,
            menu,
            index: 0,
            history: vec![]
        }
    }

    pub fn is_selected(&self, entry: &MenuEntry) -> bool {
        &self.menu.entries[self.index] == entry
    }

    pub fn activate(&mut self) -> Result<(), EventError> {
        let entry = self.menu.entries[self.index].action.clone();
        match entry {
            MenuEntryAction::Menu { menu } => {
                Ok(self.open_menu(&menu))
            },
            MenuEntryAction::BuiltIn(BuiltInAction::Exit) => {
                Err(EventError::Exit)
            },
            _ => Ok(())
        }
    }

    fn open_menu(&mut self, menu_id:&String) {
        self.history.push((self.menu.clone(), self.index));
        self.menu = self.config.menus[menu_id].clone();
        self.index = 0;
    }

    pub fn back(&mut self) {
        if let Some(x) = self.history.pop() {
            self.menu = x.0;
            self.index = x.1;
        }
    }

    pub fn move_cursor(&mut self, inc: i32) {
        let new_index = self.index as i32 + inc;
        if new_index >= 0 && new_index < self.menu.entries.len() as i32 {
            self.index = new_index as usize;
        }
    }

    pub fn iter_fwd(&self) -> impl DoubleEndedIterator<Item = &MenuEntry> {
        self.menu.entries.iter()
            .skip(self.index)
    }

    pub fn iter_rev(&self) -> impl DoubleEndedIterator<Item = &MenuEntry> {
        self.menu.entries.iter()
            .take(self.index)
            .rev()
    }
}
