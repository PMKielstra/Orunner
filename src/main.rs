use std::path::PathBuf;

use clap::Parser;

mod config;
mod cli;
mod utils;
use utils::*;

fn main() {
    let cli = cli::Cli::parse();
    let mut config = config::load();

    let (selected_profile_name, profile_readout) = match &cli.profile {
        None => (&config.default_profile, "default profile".to_string()),
        Some(prof_name) => (prof_name, format!("profile {}", prof_name))
    };
    let prof_binding = &mut config.profiles.get_mut(selected_profile_name);
    let selected_profile = match prof_binding.as_mut() {
        None => {println!("The profile you selected does not exist."); return;}
        Some(p) => p
    };

    // All commands are implemented as individual functions.
    // They return true if they've updated the config, and false otherwise.
    let dirty_config = match &cli.command {
        cli::Commands::Profile { subcommand } => routine_profile(&mut config, subcommand),
        cli::Commands::Path { subcommand } => routine_path(selected_profile, &profile_readout, subcommand),
        cli::Commands::Prefix { subcommand } => routine_prefix(selected_profile, &profile_readout, subcommand),
        cli::Commands::MakeCommand { shell_command } => routine_make_command(selected_profile, shell_command),
    };

    if dirty_config {
        match config::store(config) {
            Ok(_) => (),
            Err(e) => println!("Error storing config!  Any changes made may not have been saved.  {e}")
        }
    }
}

/// orunner profile -- create, delete, and list profiles.
fn routine_profile(config: &mut config::Config, command: &Option<cli::EditProfile>) -> bool {
    match command {
        // No command given -- default to listing profiles.
        None => {
            if config.profiles.len() > 0 {
                println!("{} [Default]", config.default_profile);
                for (name, _profile) in config.profiles.iter() {
                    if *name != config.default_profile {
                        println!("{}", name);
                    }
                }
            }
            return false;
        }

        // Add a profile.  Optionally, set it as the default.
        Some(cli::EditProfile::Add { default, name }) => {
            if config.profiles.contains_key(name) {
                println!("Profile {} already exists!", name);
                return false;
            }
            config.profiles.insert(name.to_string(), Default::default());
            if *default {
                config.default_profile = name.to_string()
            }
            println!("Added profile {}{}.", name, (if *default {" and set as default"} else {""}));
        }

        // Remove a profile.  Does not allow removal of the default profile.
        Some(cli::EditProfile::Remove { name }) => {
            if !config.profiles.contains_key(name) {
                println!("Profile {} does not exist!", name);
                return false;
            }
            if config.default_profile == *name {
                println!("Cannot erase default profile!");
                return false;
            }
            config.profiles.remove(name);
            println!("Removed profile {}.", name);
        }

        // Set an existing profile as the default.
        Some(cli::EditProfile::SetDefault { name }) => {
            if !config.profiles.contains_key(name) {
                println!("Profile {} does not exist!", name);
                return false;
            }
            config.default_profile = name.to_string();
            println!("Set profile {} as default.", name);
        }
    }

    return true;
}

/// orunner path -- list, add and delete paths from profiles.
fn routine_path(selected_profile: &mut config::Profile, profile_readout: &String, command: &Option<cli::AddRemovePath>) -> bool {
    match command {
        // No command given -- default to listing paths.
        None => {
            for path in &selected_profile.paths {
                println!("{}", path);
            }
            return false;
        }

        // Add a path.
        Some(cli::AddRemovePath::Add { force, add_relative, path }) => {
            // User should have to force Orunner to add a path that doesn't exist.
            if !force && !path.exists() {
                println!("Path does not exist.  Try again with --force if you really want to add it.");
                return false;
            }
            let canonical = pathbuf_to_string(path, !add_relative);
            if selected_profile.paths.contains(&canonical) {
                println!("Path already registered.");
                return false;
            }
            println!("Added path {} to {}.", canonical, profile_readout);
            selected_profile.paths.push(canonical);
        }

        // Remove a path.
        Some(cli::AddRemovePath::Remove { path }) => {
            // It's more of a hassle to remove absolute and relative paths separately than it is to worry about removing the wrong one,
            // so just check for both and remove whichever comes up.
            let canonical = pathbuf_to_string(path, true);
            let relative = pathbuf_to_string(path, false);
            match selected_profile.paths.iter().position(|x| *x == canonical || *x == relative) {
                None => { println!("Path not found in {}.", profile_readout); return false; }
                Some(i) => { selected_profile.paths.remove(i); }
            }
        }
    }

    return true;
}

/// orunner prefix -- display, set, or clear the prefix for a profile.
fn routine_prefix(selected_profile: &mut config::Profile, profile_readout: &String, command: &Option<cli::SetClear>) -> bool {
    let prefix_display = if selected_profile.prefix == "" { "no prefix".to_string() } else { format!("prefix {}", selected_profile.prefix) };

    match command {
        // No command given -- default to displaying the prefix.
        None => {
            let mut uppercase_profile_readout = profile_readout.clone();
            first_letter_uppercase(&mut uppercase_profile_readout);
            println!("{} has {}", uppercase_profile_readout, prefix_display);
            return false;
        }

        // Set the prefix.
        Some(cli::SetClear::Set{ value }) => {
            selected_profile.prefix = value.to_string();
            println!("Set {} prefix to {} (formerly {}).", profile_readout, value, prefix_display);
        }

        // Clear the prefix.
        Some(cli::SetClear::Clear) => {
            selected_profile.prefix = "".to_string();
            println!("Cleared {} prefix (formerly {}).", profile_readout, prefix_display);
        }
    }
    return true;
}

/// orunner make-command -- output a shell script, line by line, that cds into each of the profile's folders and executes the given command.
fn routine_make_command(selected_profile: &config::Profile, shell_command: &Vec<String>) -> bool {
    let full_command = format!("{} {}", selected_profile.prefix, shell_command.join(" "));

    for path in &selected_profile.paths {
        let canonical_path = pathbuf_to_string(&PathBuf::from(path), true); // Technically we only need to canonicalize relative paths here, but this is so quick it's easier to do it for all of them.
        println!("cd \"{}\"\n printf \"$(tput bold){}$(tput sgr0)\\n\"\n{}", canonical_path, canonical_path, full_command);
    }

    return false;
}