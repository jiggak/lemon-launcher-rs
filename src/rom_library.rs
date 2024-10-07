use std::path::Path;

use anyhow::Result;
use rusqlite::{params, Connection};

pub struct RomLibrary {
    db: Connection
}

pub struct Rom {
    pub name: String,
    pub title: String,
    pub category: String
}

impl RomLibrary {
    pub fn open(db_file: impl AsRef<Path>) -> Result<Self> {
        let db = Connection::open(db_file)?;

        db.execute(include_str!("roms_table.sql"), ())?;

        Ok(RomLibrary { db })
    }

    pub fn clear(&self) -> Result<()> {
        self.db.execute("delete from roms", ())?;
        Ok(())
    }

    pub fn add_rom(&self, rom: &Rom) -> Result<()> {
        self.db.execute(
            "insert into roms (name, title, genre) values (?1, ?2, ?3)",
            params![rom.name, rom.title, rom.category]
        )?;
        Ok(())
    }
}