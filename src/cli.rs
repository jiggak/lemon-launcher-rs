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

pub use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to lemon launcher config file
    /// [default: $LL_CONFIG_HOME/config.toml or $XDG_CONFIG_HOME/lemon-launcher/config.toml]
    #[arg(long, verbatim_doc_comment)]
    pub config: Option<PathBuf>,

    /// Path to menu file
    /// [default: $LL_CONFIG_HOME/menu.toml or $XDG_CONFIG_HOME/lemon-launcher/menu.toml]
    #[arg(long, verbatim_doc_comment)]
    pub menu: Option<PathBuf>,

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
    },

    /// Make keymap interactively and write to file
    Keymap {
        file_path: Option<PathBuf>
    }
}
