// Copyright (c) 2018 FrictionlessPortals and the frictionlessbot contributors
// See the README.md file at the top-level directory of this distribution.
// This project is licensed under the license listed in the github project.

use chrono::offset::Local;
use fern::colors::{Color, ColoredLevelConfig};
use fern::{log_file, Dispatch, InitError};
use log::LevelFilter;
use std::io::stdout;

// Method for setting up a logger system using fern.
pub fn setup_log_system(verbosity: u64) -> Result<(), InitError> {
    // Create the base configuration for the system.
    let mut base_config = Dispatch::new();

    // Set all the colors for the different levels.
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Blue);

    // Change the output level from the variable.
    base_config = match verbosity {
        0 => base_config.level(LevelFilter::Info),
        1 => base_config.level(LevelFilter::Warn),
        2 => base_config.level(LevelFilter::Debug),
        3 => base_config.level(LevelFilter::Error),
        _ => base_config.level(LevelFilter::Info),
    };

    // Log the console output to a file called "log.log".
    let logging_file = Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(log_file("res/log.log")?);

    // Setup console output for logging.
    let stdout_config = Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .chain(stdout());

    // Chain everything together and build.
    base_config
        .chain(logging_file)
        .chain(stdout_config)
        .apply()?;

    // Finish by displaying that the logging system finished.
    info!("Logging system finished setting up");

    Ok(())
}
