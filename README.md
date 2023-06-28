# Orunner
Run all your commands in all your folders.  Myrepos for generalists.

## Installation
Put `orunner` and `orc` somewhere in your path.  Alternatively, `cargo build` will get you `orunner`, and `orc` is just a shell script.

## Usage
Orunner runs on _profiles_.  A profile has a list of paths.  `orc -p profile_name git pull` or `orc @profile_name git pull` will execute `git pull` in every path associated with `profile_name`.

Profiles also have _prefixes_.  Each has exactly one.  If the prefix for `git_prof` is `"git"`, then you can just run `orc @git_prof pull` and the prefix will be prepended, to make the eventual command `git pull`.

One profile is always the default.  If you don't specify which one you want, the default will be used.

The `orc` command is used for actually executing commands in profiles, but all profile management is done through `orunner`.  This allows you to add and remove profiles and paths, and set and clear prefixes.  Run `orunner help` for more information.

By default, `orunner` adds paths _absolutely_ -- that is, they are canonicalized at the time they are added.  This way, you can enroll whatever folder you're in with just `orunner path add .`.  However, if you want a profile that always uses the folder from which you called `orc`, you can use the `--add-relative` option with `orunner path add`.
