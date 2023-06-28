use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
pub struct Cli {
    /// Select a profile, or leave blank to use the default.
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
    /// View, set, or clear a profile's prefix.  The prefix is prepended to every command run.  One common prefix is "git".
    Prefix {
        #[command(subcommand)]
        subcommand: Option<SetClear>
    },
    /// Prepare a script to run a shell command at every path in a profile.  You should not run this -- use orc instead.
    #[clap(trailing_var_arg=true)]
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
    /// Set the global default profile.
    SetDefault {
        name: String
    }
}

#[derive(Subcommand)]
pub enum AddRemovePath {
    /// Add a path to the selected profile.
    Add {
        /// Add a path that may not exist.
        #[arg(long)]
        force: bool,

        /// Add the path relatively.  Relative paths are evaluated at runtime based on the current working directory.
        #[arg(long)]
        add_relative: bool,

        path: PathBuf
    },
    /// Remove a path from the selected profile.  Will match either absolute or relative paths.
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
