// Copyright (c) 2018 FrictionlessPortals and the frictionlessbot contributors
// See the README.md file at the top-level directory of this distribution.
// This project is licensed under the license listed in the github project.

use chrono::Local;
use serenity::model::channel::EmbedAuthor;
use serenity::model::gateway::Game;
use serenity::utils::Colour;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// Command to change the current playing message.
// # Example #
// ~game test
// => playing test
command!(game(ctx, _msg, args) {
    let name = args.full();
    ctx.set_game(Game::playing(&name));

    info!("Set current playing message to: {}", &name);
});

// Command to display the ping between command execution.
// # Example #
// ~ping
// => 255ms
command!(ping(ctx, _msg, args) {
    let start = Local::now();
    let msg = _msg.reply("0");
    let end = Local::now();

    if let Ok(mut m) = msg {
        let ms = end.signed_duration_since(start).num_milliseconds();
        let _ = m.edit(|m| m.content(&format!("Pong, this ping took {} milliseconds", ms)));
    }
});

// Command to display the details of the bot.
command!(about(ctx, msg, args){

    if let Err(why) = msg.channel_id.send_message(|m| m
        .embed(|e| e
        .color(Colour::blurple())
        .author(|a| a
            .name("Frictionless Bot"))
        .title("About")
        .description("This bot is written purely in Rust, compiled with the latest compiler and shipped out to you. \
                      Find out more on our github! \
                      Created by FrictionlessPortals")
        .field("Version", format!("{}", VERSION), false))){
        error!("Error sending message: {:?}", why);
    }
});
