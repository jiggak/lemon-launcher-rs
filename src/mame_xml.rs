use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

use anyhow::Result;
use quick_xml::de::from_reader;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Mame {
    #[serde(rename="machine")]
    pub machines: Vec<Machine>
}

impl Mame {
    pub fn load_xml(file_path: impl AsRef<Path>) -> Result<Self> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        Ok(from_reader(reader)?)
    }

    pub fn load_xml_map(file_path: impl AsRef<Path>) -> Result<HashMap<String, Machine>> {
        let root = Self::load_xml(file_path)?;
        let machine_map = root.machines
            .into_iter()
            .map(|m| (m.name.clone(), m))
            .collect();
        Ok(machine_map)
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Machine {
    #[serde(rename="@name")]
    pub name: String,

    #[serde(rename="@cloneof")]
    pub clone_of: Option<String>,

    #[serde(rename="@romof")]
    pub rom_of: Option<String>,

    #[serde(rename="@ismechanical")]
    pub is_mechanical: Option<String>,

    #[serde(rename="@isdevice")]
    pub is_device: Option<String>,

    pub description: String,

    pub year: Option<String>,

    pub manufacturer: Option<String>
}

/* This is slightly faster than using serde
impl Machine {
    pub fn load_xml(xml_path: impl AsRef<Path>) -> Result<HashMap<String, Machine>> {
        let mut reader = Reader::from_file(xml_path)?;
        let mut buf = Vec::new();

        let mut result = HashMap::new();

        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Eof => break,
                Event::Start(element) => {
                    if element.name().as_ref() == b"machine" {
                        let machine = Self::from_element(element, &mut reader)?;
                        result.insert(machine.name.clone(), machine);
                    }
                },
                _ => ()
            }
        }

        Ok(result)
    }

    fn from_element(element: BytesStart, reader: &mut Reader<BufReader<File>>) -> Result<Self> {
        let mut name: Option<String> = None;
        let mut clone_of: Option<String> = None;
        let mut rom_of: Option<String> = None;
        let mut is_mechanical: Option<String> = None;
        let mut is_device: Option<String> = None;

        let mut description: Option<String> = None;
        let mut year: Option<String> = None;
        let mut manufacturer: Option<String> = None;

        for attr in element.attributes() {
            let attr = attr?;

            let val = attr.decode_and_unescape_value(reader.decoder())?;

            match attr.key.as_ref() {
                b"name" => name = Some(val.to_string()),
                b"cloneof" => clone_of = Some(val.to_string()),
                b"romof" => rom_of = Some(val.to_string()),
                b"ismechanical" => is_mechanical = Some(val.to_string()),
                b"isdevice" => is_device = Some(val.to_string()),
                _ => ()
            }
        }

        let name = name.ok_or(Error::msg("name attr required"))?;

        let mut buf = Vec::new();
        let mut txt = String::new();

        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Text(e) => txt = e.unescape().unwrap().into_owned(),
                Event::End(e) => {
                    match e.name().as_ref() {
                        b"description" => description = Some(txt.clone()),
                        b"manufacturer" => manufacturer = Some(txt.clone()),
                        b"year" => year = Some(txt.clone()),
                        // This assumes the sub-elements we are interested in
                        // are always BEFORE other elements
                        _ => break
                    }
                },
                _ => ()
            }
        }

        let description = description
            .ok_or(Error::msg("description element required"))?;

        Ok({
            Machine {
                name, clone_of, rom_of, is_mechanical, is_device, description,
                year, manufacturer
            }
        })
    }
} */