use std::collections::HashMap;

use anyhow::Result;

use crate::{
    lemon_launcher::LemonError,
    menu_config::{BuiltInAction, MenuConfig, MenuEntry, MenuEntryAction, Query},
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
                let entries = self.config.menus[&menu].entries.clone();
                Ok(self.set_entries(entries))
            },
            MenuEntryAction::BuiltIn(BuiltInAction::Exit) => {
                Err(LemonError::Exit.into())
            },
            MenuEntryAction::Query { query, params } => {
                let entries = exec_query(&query, params)?;
                Ok(self.set_entries(entries))
            },
            MenuEntryAction::Exec { exec } => {
                panic!("exec action not implemented");
            },
            MenuEntryAction::Rom { rom, params } => {
                panic!("rom action not implemented");
            }
        }
    }

    fn set_entries(&mut self, entries: Vec<MenuEntry>) {
        self.history.push((self.entries.clone(), self.index));
        self.entries = entries;
        self.index = 0;
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

pub fn exec_query(query: &Query, params: Option<HashMap<String, String>>) -> Result<Vec<MenuEntry>> {
    let rom_lib = RomLibrary::open("games.sqlite")?;

    match query {
        Query::Categories => {
            let categories = rom_lib.list_categories()?;
            let entries = categories.iter()
                .map(|c| MenuEntry {
                    title: c.clone(),
                    action: MenuEntryAction::Query {
                        query: Query::Roms,
                        params: Some(HashMap::from([(String::from("genre"), c.clone())]))
                    }
                })
                .collect();
            Ok(entries)
        },
        Query::Roms => {
            // FIXME figure out how to pass params to list_roms()
            let genre = &params.unwrap()["genre"];
            let roms = rom_lib.list_roms(genre)?;
            let entries = roms.iter()
                .map(|r| MenuEntry {
                    title: r.title.clone(),
                    action: MenuEntryAction::Rom {
                        rom: r.name.clone(),
                        params: None
                    }
                })
                .collect();
            Ok(entries)
        }
    }
}
