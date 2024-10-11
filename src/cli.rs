use std::path::PathBuf;

pub use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Set the directory of lemon launcher config
    /// [default: $LL_CONFIG_HOME or $XDG_CONFIG_HOME/lemon-launcher]
    #[arg(short, verbatim_doc_comment)]
    pub data_dir: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run lemon launcher graphical interface
    Launch,

    /// Build rom database
    Scan {
        mame_xml: PathBuf,
        genre_ini: PathBuf,
        roms_dir: PathBuf
    }
}
