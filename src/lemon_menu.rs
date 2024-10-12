use std::process::Command;

use anyhow::Result;

use crate::{
    lemon_config::MameCommand, lemon_launcher::LemonError,
    menu_config::{BuiltInAction, MenuConfig, MenuEntry, MenuEntryAction, Query},
    rom_library::RomLibrary
};

pub struct LemonMenu {
    config: MenuConfig,
    mame_cmd: MameCommand,
    entries: Vec<MenuEntry>,
    index: usize,
    history: Vec<(Vec<MenuEntry>, usize)>
}

impl LemonMenu {
    pub fn new(config: MenuConfig, mame_cmd: MameCommand) -> Self {
        let entries = config.main.entries.clone();
        LemonMenu {
            config,
            mame_cmd,
            entries,
            index: 0,
            history: vec![]
        }
    }

    pub fn selected(&self) -> &MenuEntry {
        &self.entries[self.index]
    }

    pub fn is_selected(&self, entry: &MenuEntry) -> bool {
        self.selected() == entry
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
            MenuEntryAction::Query(query) => {
                let entries = exec_query(&query)?;
                Ok(self.set_entries(entries))
            },
            MenuEntryAction::Exec { exec, args } => {
                exec_command(&exec, args.as_ref())
            },
            MenuEntryAction::Rom { rom, params } => {
                self.mame_cmd.exec(&rom, params.as_ref())
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

fn exec_query(query: &Query) -> Result<Vec<MenuEntry>> {
    let rom_lib = RomLibrary::open()?;

    match query {
        Query::Categories => {
            let categories = rom_lib.list_categories()?;
            let entries = categories.iter()
                .map(|c| MenuEntry {
                    title: c.clone(),
                    action: MenuEntryAction::Query(
                        Query::Roms { genre: Some(c.clone()) }
                    ),
                    screenshot: None
                })
                .collect();
            Ok(entries)
        },
        Query::Roms { genre } => {
            let roms = rom_lib.list_roms(genre.as_ref())?;
            let entries = roms.iter()
                .map(MenuEntry::from)
                .collect();
            Ok(entries)
        },
        Query::Favourites { count } => {
            let roms = rom_lib.list_favourites(*count)?;
            let entries = roms.iter()
                .map(MenuEntry::from)
                .collect();
            Ok(entries)
        },
        Query::Popular { count } => {
            let roms = rom_lib.list_most_played(*count)?;
            let entries = roms.iter()
                .map(MenuEntry::from)
                .collect();
            Ok(entries)
        }
    }
}

fn exec_command(cmd: &String, args: Option<&Vec<String>>) -> Result<()> {
    let mut cmd = Command::new(cmd);

    if let Some(args) = args {
        cmd.args(args);
    }

    cmd.spawn()?;

    Ok(())
}

impl MameCommand {
    pub fn exec(&self, rom: &String, rom_params: Option<&String>) -> Result<()> {
        let mut cmd = Command::new(&self.cmd);

        if let Some(args) = &self.args {
            cmd.args(args);
        }

        if let Some(args) = rom_params {
            cmd.arg(args);
        }

        cmd.arg(rom);

        cmd.spawn()?;

        Ok(())
    }
}
