# Orunner Spec

* `or profile`
  
  * With no arguments: list profiles
  
  * With the name of a profile: list associated paths and aliases
  
  * `add` and `remove`
    
    * `add -d` or `add --default` also carries out `set-default`
  
  * `set-default`

* `or alias`
  
  * With no arguments: list default profile's aliases
  
  * `-p`/`--profile`/`@profile`: list specific profile's aliases
  
  * `add` and `remove`
    
    * `remove` raises an error if the current profile doesn't have the alias to remove, but it will search for that alias in other profiles.  (In general, this is how Orcrunner handles "Not found": it looks through all profiles, suggests the ones that have it, and raises an error.  If there is only one option, it says "Run `or` or `orc` without any arguments to attempt."  This is handled through the environment variable `OR_RECOVERY`.)
  
  * `rename`

* `or path`
  
  * With no arguments: list current default profile's paths (which is the first profile unless otherwise specified)
  
  * `-p`/`--profile`/`@profile`
  
  * `add` and `remove`
    
    * `remove` searches
    
    * Expand relative paths
      
      * `--add-relative`
    
    * By default do not allow adding non-existent paths
      
      * `-f`/`--force`
    
    * If path already exists, do not add

* `or prefix`
  
  * With no arguments: print default profile's prefixes
  
  * `set` and `clear`
    
    * Warn when setting on default profile, but do it anyway
    
    * `-p` etc

* `or command`/`or run`/`orc`
  
  * Run a command in every folder in a profile
  
  * `-p`/`--profile`/`@profile` overrides environment variable `OR_PROFILE` overrides default profile from orfile
  
  * `!alias` is expanded if it's the first non-profile option in the parser
  
  * Prefix is appended to anything that's *not* an alias
  
  * Check if each path exists before running, and print error if it doesn't
