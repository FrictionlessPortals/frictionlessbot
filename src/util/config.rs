// Copyright (c) 2018 FrictionlessPortals and the frictionlessbot contributors
// See the README.md file at the top-level directory of this distribution.
// This project is licensed under the license listed in the github project.

use serde_json::from_reader;

use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub token: String,
    pub prefix: String,
    pub owner_id: u64,
}

// Parse the JSON Config file located in res/config.json into Config
pub fn parse<T: AsRef<Path>>(path: T) -> Result<Config, Box<Error>> {
    let file = File::open(path)?;

    let out = from_reader(file)?;

    Ok(out)
}
