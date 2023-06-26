use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
pub struct Cli {
    #[arg(short, long)]
    pub profile: Option<String>,

    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    /// List, add, or remove profiles.
    Profile {
        #[command(subcommand)]
        subcommand: Option<EditProfile>
    },
    /// List, add, or remove paths from a profile.
    Path {
        #[command(subcommand)]
        subcommand: Option<AddRemovePath>
    },
    /// View, set, or clear a profile's prefix.
    Prefix {
        #[command(subcommand)]
        subcommand: Option<SetClear>
    },
    /// Prepare a script to run a shell command at every path in a profile.  You should not run this -- use orc instead.
    MakeCommand {
        shell_command: Vec<String>
    }
}

#[derive(Subcommand)]
pub enum EditProfile {
    Add {
        /// Set this to be the new default profile.
        #[arg(short, long)]
        default: bool,
        name: String
    },
    Remove {
        name: String
    },
    SetDefault {
        name: String
    }
}

#[derive(Subcommand)]
pub enum AddRemovePath {
    Add {
        /// Add a path that may not exist.
        #[arg(long)]
        force: bool,

        path: PathBuf
    },
    Remove {
        path: PathBuf
    }
}

#[derive(Subcommand)]
pub enum SetClear {
    Set {
        value: String
    },
    Clear
}
