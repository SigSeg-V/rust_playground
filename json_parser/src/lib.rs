// This lib is to show how to read and write YAML

use serde::{Deserialize, Serialize};
use serde_yaml::{self};

// The easiest way to get anything useful is to create a struct with the elements in our file
// We need Serialize and Deserialize, as well as features = ["derive"] in the Cargo.toml
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub update_frequency_sec: u32,
    pub num_threads: u32, 
    pub data_sources: Vec<String>,
}

// implementing a default for the config lets us generate a sensible file if it doesn't exist where we look for it
impl Default for Config {
    fn default() -> Config {
        Config {
            data_sources: vec![
                String::from("www.google.com"),
                String::from("finance.yahoo.com"),
                String::from("www.msn.com"),
            ],
            update_frequency_sec: 30,
            num_threads: 2,
        }
    }
}

pub fn read_yaml() -> Config {
    
    // open file for reading, create one if none exist
    let file = std::fs::OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open("config.yml")
        .unwrap();
    
    serde_yaml::from_reader(file)
        .unwrap_or_else(|_| write_yaml(Config::default()))
}

pub fn write_yaml(config: Config) -> Config{

    // write to the file, creating a new one if it does not exist
    let file = std::fs::OpenOptions::new()
    .write(true)
    .create(true)
    .open("config.yml")
    .expect("Could not open file.");
    serde_yaml::to_writer(file, &config).unwrap();
    config
}