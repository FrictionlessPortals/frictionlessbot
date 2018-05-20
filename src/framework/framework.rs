// Copyright (c) 2018 FrictionlessPortals and the frictionlessbot contributors
// See the README.md file at the top-level directory of this distribution.
// This project is licensed under the license listed in the github project.

use serenity::framework::standard::{DispatchError, StandardFramework};
use serenity::model::gateway::Ready;
use serenity::model::id::UserId;
use serenity::prelude::*;
use serenity::utils::Colour;

use commands::*;
use util::config::Config;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("Connected to discord as {}", ready.user.name);
    }
}

// Construct the full framework for a client instance.
pub fn construct_framework(config: &Config) -> StandardFramework {
    StandardFramework::new()
            .configure(|c| c
                .allow_whitespace(true)
                .on_mention(true)
                .owners(vec![UserId(config.owner_id)].into_iter().collect())
                .prefix(&config.prefix))

            // Outputs to the log when a command is executed.
            .before(|ctx, msg, command_name| {
                debug!("Got command '{}' by user '{}'",
                         command_name,
                         msg.author.name);
                true
            })

            // Outputs to the log when a command has been processed.
            .after(|_, _, command_name, error| {
                match error {
                    Ok(()) => debug!("Processed command '{}'", command_name),
                    Err(why) => error!("Command '{}' returned error {:?}", command_name, why),
                }
            })

            // Handles cooldowns for discord's ratelimiter.
            .on_dispatch_error(|_ctx, msg, error| {
                if let DispatchError::RateLimited(seconds) = error {
                    if let Err(why) = msg.channel_id.send_message(|m| m
                        .embed(|e| e
                            .title(&format!("Command cooldown! Try again in {} seconds", seconds))
                            .color(Colour::red()))){
                        error!("Error sending message: {:?}", why);
                    }
                }
            })

            // Simple cooldown bucket for commands.
            .simple_bucket("common", 5)

            // General utility commands
            .command("ping", |c| c
                .bucket("common")
                .owners_only(true)
                .cmd(utils::ping))
            .command("game", |c| c
                .bucket("common")
                .owners_only(true)
                .cmd(utils::game))
            .command("about", |c| c
                .bucket("common")
                .cmd(utils::about))

            // Math commands
            .command("add", |c| c
                .bucket("common")
                .cmd(math::addition))
            .command("multiply", |c| c
                .bucket("common")
                .cmd(math::multiply))
            .command("subtract", |c| c
                .bucket("common")
                .cmd(math::subtract))
            .command("division", |c| c
                .bucket("common")
                .cmd(math::divide))
}

// Create a client instance using a bot token and return it.
pub fn create_client(token: &String) -> Client {
    info!("Generating client from token");

    let client = Client::new(&token, Handler).expect("Error creating client object");

    client
}
