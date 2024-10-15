use std::path::Path;

use anyhow::Result;
use fallible_iterator::FallibleIterator;
use rusqlite::{params, Connection, Params};

use crate::env;

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
    pub fn open() -> Result<Self> {
        Self::open_file(env::get_rom_lib_path())
    }

    pub fn open_file(db_file: impl AsRef<Path>) -> Result<Self> {
        let db = Connection::open(db_file)?;

        db.execute(include_str!("roms_table.sql"), ())?;

        Ok(RomLibrary { db })
    }

    pub fn clear(&self) -> Result<()> {
        self.db.execute("delete from roms", ())?;
        Ok(())
    }

    // pub fn add_rom(&self, rom: &Rom) -> Result<()> {
    //     self.db.execute(
    //         "insert into roms (name, title, genre, clone_of) values (?1, ?2, ?3, ?4)",
    //         params![rom.name, rom.title, rom.category, rom.clone_of]
    //     )?;
    //     Ok(())
    // }

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

    pub fn inc_play_count(&self, rom_name: &String) -> Result<()> {
        self.db.execute("
            update roms set play_count = play_count + 1
            where name = ?1
        ", [rom_name])?;
        Ok(())
    }

    pub fn toggle_favourite(&self, rom_name: &String) -> Result<()> {
        self.db.execute("
            update roms set favourite = not favourite
            where name = ?1
        ", [rom_name])?;
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

    fn roms_query<P: Params>(&self, sql: &str, params: P) -> Result<Vec<Rom>> {
        let mut stmt = self.db.prepare(sql)?;

        let rows = stmt.query(params)?;
        let roms = rows
            .map(|r| Ok(Rom {
                name: r.get(0)?,
                title: r.get(1)?,
                category: r.get(2)?,
                clone_of: None
            }))
            .collect()?;

        Ok(roms)
    }

    pub fn list_roms(&self, category: Option<&String>) -> Result<Vec<Rom>> {
        self.roms_query("
            select name, title, genre from roms
            where clone_of is null and (?1 is null or genre = ?1)
            order by title
        ", [category])
    }

    pub fn list_favourites(&self, count: u32) -> Result<Vec<Rom>> {
        self.roms_query("
            select name, title, genre from roms
            where clone_of is null and favourite = 1
            order by title
            limit ?1
        ", [count])
    }

    pub fn list_most_played(&self, count: u32) -> Result<Vec<Rom>> {
        self.roms_query("
            select name, title, genre from roms
            where clone_of is null and play_count > 0
            order by play_count desc, title
            limit ?1
        ", [count])
    }
}