use std::{fs, time::Duration};

pub struct Config {
    pub rom: Vec<u8>,
    pub tick_rate: Duration,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Self, String> {
        let rom = match args.next() {
            Some(path) => match fs::read(&path) {
                Ok(contents) => contents,
                Err(err) => return Err(format!("Failed to read rom at path '{path}'\n{err}")),
            },
            None => return Err(String::from("Missing rom path")),
        };

        let tick_rate = match args.next() {
            None => 250u64,

            Some(arg) => match arg.parse() {
                Ok(arg) => arg,
                Err(err) => {
                    return Err(format!(
                        "Tickrate should be an integer between {} and {}\n{}",
                        u64::MIN,
                        u64::MAX,
                        err
                    ))
                }
            },
        };

        Ok(Self {
            rom,
            tick_rate: Duration::from_millis(tick_rate),
        })
    }
}
