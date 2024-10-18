use std::{io, process::Command};

use anyhow::Result;

use crate::{
    lemon_config::MameCommand, lemon_screen::EventReply,
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

    pub fn selected(&self) -> Option<&MenuEntry> {
        self.entries.get(self.index)
    }

    pub fn is_selected(&self, entry: &MenuEntry) -> bool {
        self.selected()
            .map(|selected| selected == entry)
            .unwrap_or_default()
    }

    pub fn activate(&mut self) -> Result<EventReply> {
        if let Some(entry) = self.selected() {
            let action = entry.action.clone();
            match action {
                MenuEntryAction::Menu { menu } => {
                    let entries = self.config.menus[&menu].entries.clone();
                    self.set_entries(entries)
                },
                MenuEntryAction::BuiltIn(BuiltInAction::Exit) => {
                    return Ok(EventReply::Exit)
                },
                MenuEntryAction::Query(query) => {
                    let entries = query.exec()?;
                    self.set_entries(entries)
                },
                MenuEntryAction::Exec { exec, args } => {
                    exec_command(&exec, args.as_ref())?
                },
                MenuEntryAction::Rom { rom, params } => {
                    let rom_lib = RomLibrary::open()?;
                    rom_lib.inc_play_count(&rom)?;

                    self.mame_cmd.exec(&rom, params.as_ref())?
                }
            }
        }

        Ok(EventReply::Handled)
    }

    pub fn toggle_favourite(&mut self) -> Result<()> {
        if let Some(entry) = self.selected() {
            if let MenuEntryAction::Rom { rom, .. } = &entry.action {
                let rom_lib = RomLibrary::open()?;
                rom_lib.toggle_favourite(rom)?;
                self.refresh()?;
            }
        }

        Ok(())
    }

    fn set_entries(&mut self, entries: Vec<MenuEntry>) {
        self.history.push((self.entries.clone(), self.index));
        self.entries = entries;
        self.index = 0;
    }

    fn refresh(&mut self) -> Result<()> {
        if let Some((entries, index)) = self.history.last() {
            if let MenuEntryAction::Query(query) = &entries[*index].action {
                self.entries = query.exec()?;
            }
        }

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

impl Query {
    pub fn exec(&self) -> Result<Vec<MenuEntry>> {
        let rom_lib = RomLibrary::open()?;

        match self {
            Query::Categories => {
                let categories = rom_lib.list_categories()?;
                let entries = categories.iter()
                    .map(|c| MenuEntry {
                        title: c.clone(),
                        action: MenuEntryAction::Query(
                            Query::Roms { genre: Some(c.clone()) }
                        ),
                        screenshot: None,
                        details: None
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
}

fn exec_command(cmd: &String, args: Option<&Vec<String>>) -> io::Result<()> {
    let mut cmd = Command::new(cmd);

    if let Some(args) = args {
        cmd.args(args);
    }

    cmd.spawn()?;

    Ok(())
}

impl MameCommand {
    pub fn exec(&self, rom: &String, rom_params: Option<&String>) -> io::Result<()> {
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
