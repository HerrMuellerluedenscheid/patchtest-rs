use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

use serde::{Deserialize, Serialize};

use crate::checks::Level;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub levels: HashMap<String, Level>,
}

impl Config {
    pub fn get_error_level(&self, name: &String) -> &Level {
        self.levels.get(name).unwrap_or(&Level::Error)
    }
}

pub fn load_config(filename: &Path) -> Config {
    let f = File::open(filename).expect("failed loading failed");
    let reader = BufReader::new(f);
    let config: Config = serde_yaml::from_reader(reader).expect("failed loading config");
    config
}

pub fn print_config() {
    let levels: HashMap<String, Level> = HashMap::new();
    let config = Config { levels };
    println!("{}", serde_yaml::to_string(&config).unwrap());
}
