use serde_json::{from_reader, json, to_string_pretty, Value};
use std::fs::{create_dir, File};
use std::io::BufReader;
use std::io::Write;
use std::path::Path;

pub struct Config {
    hotkey: u32,
}
fn write_default_config(path: &Path) {
    let config = json!({
       "hotkey": 19
    });

    let mut file = File::create(path).expect("Unsable to open file");

    file.write_all(to_string_pretty(&config).unwrap().as_bytes());
}

impl Config {
    pub fn read_config() -> Config {
        let mut dir = std::env::home_dir().expect("HOMEDIR undefined");

        dir.push(".xim-connect");

        if !dir.exists() {
            create_dir(&dir).expect("Could not create dir");
        }

        if !dir.is_dir() {
            panic!("{} is not a directory", dir.to_str().unwrap());
        }

        dir.push("config.json");
        if !dir.exists() {
            write_default_config(&dir);
        }

        if !dir.is_file() {
            panic!("{} is not a directory", dir.to_str().unwrap());
        }

        let config = File::open(dir).expect("Cannot open file");
        let buffer = BufReader::new(config);
        let json: Value = from_reader(buffer).unwrap();
        let hotkey = json
            .get("hotkey")
            .expect("hotkey not defined")
            .as_u64()
            .expect("hotkey not an integer");
        Config {
            hotkey: hotkey as u32,
        }
    }
}
