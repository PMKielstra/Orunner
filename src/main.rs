use std::path::PathBuf;

use clap::Parser;

mod config;
mod cli;

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

fn routine_profile(config: &mut config::Config, command: &Option<cli::EditProfile>) -> bool {
    match command {
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

fn pathbuf_to_string(p: &PathBuf) -> String {
    return (|| { p.canonicalize().ok()?.into_os_string().into_string().ok() }) ().unwrap()
}

fn routine_path(selected_profile: &mut config::Profile, profile_readout: &String, command: &Option<cli::AddRemovePath>) -> bool {
    match command {
        None => {
            for path in &selected_profile.paths {
                println!("{}", path);
            }
            return false;
        }
        Some(cli::AddRemovePath::Add { force, path }) => {
            if !force && !path.exists() {
                println!("Path does not exist.  Try again with --force if you really want to add it.");
                return false;
            }
            let canonical = pathbuf_to_string(path);
            if selected_profile.paths.contains(&canonical) {
                println!("Path already registered.");
                return false;
            }
            println!("Added path {} to {}.", canonical, profile_readout);
            selected_profile.paths.push(canonical);
        }
        Some(cli::AddRemovePath::Remove { path }) => {
            let canonical = pathbuf_to_string(path);
            match selected_profile.paths.iter().position(|x| *x == canonical) {
                None => { println!("Path does not exist."); return false; }
                Some(i) => { selected_profile.paths.remove(i); }
            }
        }
    }
    return true;
}

fn first_letter_uppercase(s: &mut str) {
    if let Some(r) = s.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
}

fn routine_prefix(selected_profile: &mut config::Profile, profile_readout: &String, command: &Option<cli::SetClear>) -> bool {
    match command {
        None => {
            let mut uppercase_profile_readout = profile_readout.clone();
            first_letter_uppercase(&mut uppercase_profile_readout);
            println!("{} has {}", uppercase_profile_readout, if selected_profile.prefix == "" { "no prefix".to_string() } else { format!("prefix {}", selected_profile.prefix) });
            return false;
        }
        Some(cli::SetClear::Set{ value }) => {
            selected_profile.prefix = value.to_string();
            println!("Set {} prefix to {}.", profile_readout, value);
        }
        Some(cli::SetClear::Clear) => {
            selected_profile.prefix = "".to_string();
            println!("Cleared {} prefix.", profile_readout)
        }
    }
    return true;
}

fn routine_make_command(selected_profile: &config::Profile, shell_command: &Vec<String>) -> bool {
    let full_command = format!("{} {}", selected_profile.prefix, shell_command.join(" "));
    for path in &selected_profile.paths {
        println!("cd \"{}\"\n printf \"$(tput bold){}$(tput sgr0)\\n\"\n {}", path, path, full_command);
    }
    return false;
}