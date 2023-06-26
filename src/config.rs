use std::collections::BTreeMap;

use confy::ConfyError;
use serde_derive::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Profile {
    pub prefix: String,
    pub paths: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub default_profile: String,
    pub profiles: BTreeMap<String, Profile>,
}

const DEFAULT_PROF_NAME: &str = "default";

impl ::std::default::Default for Config {
    fn default() -> Config {
        let default_prof = Profile {prefix: "".to_string(), paths: Vec::new()};
        let prof_map = BTreeMap::from([(DEFAULT_PROF_NAME.to_string(), default_prof)]);
        Config {default_profile: DEFAULT_PROF_NAME.to_string(), profiles: prof_map}
    }
}

const NAME: &str = "orunner";

pub fn load() -> Config {
    return confy::load(NAME, None).unwrap();
}

pub fn store(cfg: Config) -> Result<(), ConfyError> {
    return confy::store(NAME, None, cfg);
}