use std::path::PathBuf;

pub use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    pub data_dir: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>
}

#[derive(Subcommand)]
pub enum Commands {
    Launch,
    Scan {
        mame_xml: PathBuf,
        genre_ini: PathBuf,
        roms_dir: PathBuf
    }
}
