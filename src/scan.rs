use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::Path
};

use anyhow::{Error, Result};
use configparser::ini::Ini;

use crate::rom_library::{Rom, RomLibrary};

pub fn scan(mame_list: &Path, genre_ini: &Path, roms_dir: &Path) -> Result<()> {
    let titles = parse_mame_list(mame_list)?;
    let categories = parse_genre_ini(genre_ini)?;

    let mut rom_meta: HashMap<&String, Rom> = HashMap::new();

    let category_map = categories.get_map_ref();
    for (category, category_roms) in category_map {
        for (rom, _) in category_roms {
            if let Some(title) = titles.get(rom) {
                rom_meta.insert(rom, Rom {
                    name: rom.clone(),
                    title: title.clone(),
                    category: category.clone()
                });
            } else {
                println!("Title for rom {rom} not found");
            }
        }
    }

    let mut roms = vec![];

    for dir_entry in fs::read_dir(roms_dir)? {
        let dir_entry = dir_entry?;

        // let rom_file_name = dir_entry.file_name();

        let dir_entry_path = dir_entry.path();
        let dir_entry_path = dir_entry_path.with_extension("");
        let rom_name = dir_entry_path
            .file_name()
            .ok_or(Error::msg("Rom dir entry must be file"))?
            .to_string_lossy()
            .to_string();

        if let Some(rom) = rom_meta.get(&rom_name) {
            roms.push(rom);
        } else {
            println!("Rom meta data not found {rom_name}");
        }
    }

    let rom_lib = RomLibrary::open("games.sqlite")?;
    rom_lib.clear()?;
    rom_lib.add_roms(&roms)?;

    println!("Added {} roms to library", roms.len());

    Ok(())
}

fn parse_mame_list(mame_list: &Path) -> Result<HashMap<String, String>> {
    let file = File::open(mame_list)?;
    let reader = BufReader::new(file);

    let mut title_map = HashMap::new();

    for line in reader.lines() {
        let line = parse_mame_list_line(line?)?;
        if line.0 == "Name:" {
            // Skip header row
            continue;
        }

        title_map.insert(line.0, line.1);
    }

    Ok(title_map)
}

fn parse_mame_list_line(line: String) -> Result<(String, String)> {
    line.split_once(' ')
        .ok_or(Error::msg("Expected space delimited line in mame list file"))
        .map(|x| (
            x.0.into(),
            // trim leading whitespace from title and remove quotes
            x.1.trim().trim_matches('"').into()
        ))
}

fn parse_genre_ini(genre_ini: &Path) -> Result<Ini> {
    // new_cs() to preserve case of category names
    let mut genre = Ini::new_cs();

    genre.load(genre_ini)
        .map_err(|e| Error::msg(e))?;

    Ok(genre)
}
