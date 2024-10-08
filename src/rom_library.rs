use std::path::Path;

use anyhow::Result;
use fallible_iterator::FallibleIterator;
use rusqlite::{params, Connection};

pub struct RomLibrary {
    db: Connection
}

pub struct Rom {
    pub name: String,
    pub title: String,
    pub category: String,
    pub clone_of: Option<String>
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
            "insert into roms (name, title, genre, clone_of) values (?1, ?2, ?3, ?4)",
            params![rom.name, rom.title, rom.category, rom.clone_of]
        )?;
        Ok(())
    }

    pub fn add_roms(&self, roms: &Vec<&Rom>) -> Result<()> {
        // this magic makes batch insert take seconds vs dozens of minutes
        // https://github.com/avinassh/fast-sqlite3-inserts/blob/cbe53fd/src/bin/basic_prep.rs
        self.db.execute_batch("
            PRAGMA journal_mode = OFF;
            PRAGMA synchronous = 0;
            PRAGMA cache_size = 1000000;
            PRAGMA locking_mode = EXCLUSIVE;
            PRAGMA temp_store = MEMORY;
        ")?;

        let mut stmt = self.db.prepare("
            insert into roms (name, title, genre, clone_of)
            values (?1, ?2, ?3, ?4)
        ")?;

        for rom in roms {
            stmt.execute(params![rom.name, rom.title, rom.category, rom.clone_of])?;
        }

        Ok(())
    }

    pub fn list_categories(&self) -> Result<Vec<String>> {
        let mut stmt = self.db.prepare("select genre from roms where clone_of is null group by genre")?;
        let rows = stmt.query([])?;
        let categories = rows
            .map(|r| r.get(0))
            .collect()?;
        Ok(categories)
    }

    pub fn list_roms(&self, category: &String) -> Result<Vec<Rom>> {
        let mut stmt = self.db.prepare("
            select name, title, genre, clone_of
            from roms
            where clone_of is null and genre = ?1
            order by title
        ")?;

        let rows = stmt.query([category])?;
        let roms = rows
            .map(|r| Ok(Rom {
                name: r.get(0)?,
                title: r.get(1)?,
                category: r.get(2)?,
                clone_of: r.get(3)?
            }))
            .collect()?;

        Ok(roms)
    }
}