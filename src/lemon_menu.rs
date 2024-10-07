use anyhow::Result;

use crate::{
    lemon_launcher::LemonError,
    menu_config::{BuiltInAction, Menu, MenuConfig, MenuEntries, MenuEntry, MenuEntryAction},
    rom_library::RomLibrary
};

pub struct LemonMenu {
    config: MenuConfig,
    entries: Vec<MenuEntry>,
    index: usize,
    history: Vec<(Vec<MenuEntry>, usize)>
}

impl LemonMenu {
    pub fn new(config: MenuConfig) -> Self {
        let entries = config.main.entries.clone();
        LemonMenu {
            config,
            entries,
            index: 0,
            history: vec![]
        }
    }

    pub fn is_selected(&self, entry: &MenuEntry) -> bool {
        &self.entries[self.index] == entry
    }

    pub fn activate(&mut self) -> Result<()> {
        let entry = self.entries[self.index].action.clone();
        match entry {
            MenuEntryAction::Menu { menu } => {
                self.open_menu(&menu)
            },
            MenuEntryAction::BuiltIn(BuiltInAction::Exit) => {
                Err(LemonError::Exit.into())
            },
            _ => Ok(())
        }
    }

    fn open_menu(&mut self, menu_id:&String) -> Result<()> {
        self.history.push((self.entries.clone(), self.index));
        self.entries = self.config.menus[menu_id].get_entires()?;
        self.index = 0;
        Ok(())
    }

    pub fn back(&mut self) {
        if let Some(x) = self.history.pop() {
            self.entries = x.0;
            self.index = x.1;
        }
    }

    pub fn move_cursor(&mut self, inc: i32) {
        let new_index = self.index as i32 + inc;
        if new_index >= 0 && new_index < self.entries.len() as i32 {
            self.index = new_index as usize;
        }
    }

    pub fn iter_fwd(&self) -> impl DoubleEndedIterator<Item = &MenuEntry> {
        self.entries.iter()
            .skip(self.index)
    }

    pub fn iter_rev(&self) -> impl DoubleEndedIterator<Item = &MenuEntry> {
        self.entries.iter()
            .take(self.index)
            .rev()
    }
}

impl Menu {
    pub fn get_entires(&self) -> Result<Vec<MenuEntry>> {
        match &self.entries {
            MenuEntries::Static(entries) => Ok(entries.clone()),
            MenuEntries::Query { query } => {
                let rom_lib = RomLibrary::open("games.sqlite")?;
                let categories = rom_lib.list_categories()?;
                let entries = categories.iter()
                    .map(|c| MenuEntry {
                        title: c.clone(),
                        action: MenuEntryAction::Menu { menu: "foo".to_string() }
                    })
                    .collect();
                Ok(entries)
            }
        }
    }
}