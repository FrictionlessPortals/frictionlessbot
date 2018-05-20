// Copyright (c) 2018 FrictionlessPortals and the frictionlessbot contributors
// See the README.md file at the top-level directory of this distribution.
// This project is licensed under the license listed in the github project.

extern crate chrono;
extern crate fern;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate log;
#[macro_use]
extern crate serenity;
#[macro_use]
extern crate serde_derive;

mod commands;
mod framework;
mod util;

fn main() {
    // Setup logging system.
    util::logging::setup_log_system(0).unwrap();

    // Parse the config into the config struct.
    let config = util::config::parse("res/config.json").unwrap();

    // Create a client instance from a token.
    let mut client = framework::framework::create_client(&config.token);

    // Create the main framework for attaching to the client instance.
    let framework = framework::framework::construct_framework(&config);
    client.with_framework(framework);

    // Start the bot up!
    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}
