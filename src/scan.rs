use std::{collections::HashMap, fs, path::Path};

use anyhow::{Error, Result};
use configparser::ini::Ini;

use crate::{
    mame_xml::Mame,
    rom_library::{Rom, RomLibrary}
};

pub fn scan(mame_xml: &Path, genre_ini: &Path, roms_dir: &Path) -> Result<()> {
    let mame_db = Mame::load_xml_map(mame_xml)?;
    let categories = parse_genre_ini(genre_ini)?;

    let mut rom_meta: HashMap<&String, Rom> = HashMap::new();

    let category_map = categories.get_map_ref();
    for (category, category_roms) in category_map {
        for (rom, _) in category_roms {
            if let Some(machine) = mame_db.get(rom) {
                rom_meta.insert(rom, Rom {
                    name: rom.clone(),
                    title: machine.description.clone(),
                    category: category.clone(),
                    clone_of: machine.clone_of.clone(),
                    is_favourite: false,
                    year: machine.year.clone(),
                    manufacturer: machine.manufacturer.clone()
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

    let rom_lib = RomLibrary::open()?;
    rom_lib.clear()?;
    rom_lib.add_roms(&roms)?;

    println!("Added {} roms to library", roms.len());

    Ok(())
}

fn parse_genre_ini(genre_ini: &Path) -> Result<Ini> {
    // new_cs() to preserve case of category names
    let mut genre = Ini::new_cs();

    genre.load(genre_ini)
        .map_err(|e| Error::msg(e))?;

    Ok(genre)
}
