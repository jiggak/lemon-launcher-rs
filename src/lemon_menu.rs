/*
 * Lemon Launcher - SDL based MAME frontend for arcade cabinets
 * Copyright (C) 2024 Josh Kropf <josh@slashdev.ca>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::path::PathBuf;

use anyhow::Result;

use crate::{
    menu_config::{MenuConfig, MenuEntry, MenuEntryAction, MenuEntryDetail, Query},
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

    pub fn selected(&self) -> Option<&MenuEntry> {
        self.entries.get(self.index)
    }

    pub fn selected_screenshot(&self) -> Option<&PathBuf> {
        self.selected().and_then(|entry| entry.screenshot.as_ref())
    }

    pub fn selected_detail(&self) -> Option<&MenuEntryDetail> {
        self.selected().and_then(|x| x.details.as_ref())
    }

    pub fn is_selected(&self, entry: &MenuEntry) -> bool {
        self.selected()
            .map(|selected| selected == entry)
            .unwrap_or_default()
    }

    pub fn open_menu(&mut self, menu_name: &str) {
        let entries = self.config.menus[menu_name].entries.clone();
        self.set_entries(entries)
    }

    pub fn open_query(&mut self, query: &Query) -> Result<()> {
        let entries = query.exec()?;
        self.set_entries(entries);
        Ok(())
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
