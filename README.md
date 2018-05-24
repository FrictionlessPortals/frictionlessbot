# frictionlessbot

FrictionlessBot is a general-purpose [Discord][discord] bot written in
[Rust][rust-lang] using the [serenity.rs][serenity-rs] Discord API.

# Table of Contents

* [Usage](#usage)
* [Configuration](#configuration)
* [Commands](#commands)
* [Credits](#credits)
* [License](#license)

## Usage
### Invitation

The easiest way to use this bot is to invite it to your Discord server. You will
need to host the bot yourself, instructions for each OS will be available soon.

To invite the bot to your server you need your invite link. You must have the
**Manage Server** permission for that server to invite the bot.

If you have any questions or feedback, feel free to [open an
issue][new-issue] in this repo.

### Personal Setup

A personal instance of the bot needs to be configured.

Setup a bot account:

1. Go to Discord Developers [My Applications][my-applications] page
2. Click "New Application"
3. Enter a name for your application (this is not your bot username) in the
   "App Name" field
4. (Optional) Add a description and/or icon for your bot (the icon will be your
   bot's profile picture)
5. Click "Create Application"

Create a bot user:

1. On the page for your application, click "Create a Bot User"
2. Click "Yes, do it!" when asked to confirm
3. Now that your bot is created, press "click to reveal" on the application page
   to see your bot's token.
4. Copy this value, and paste it into the file [`example.config.json`][json-example]
   (included in this repository), in the place of the existing
   `token` value.
5. Rename the `example.config.json` file to `config.json`, and modify any other configuration
   options you wish.

Run an instance of the bot, either by running a pre-built executable (coming
soon!), or by compiling the bot yourself (instructions coming soon!).

## Configuration

Currently, FrictionlessBot reads its configuration from a json file. It is assumed
that you have then loaded the necessary configuration options into the
file prior to running the bot.

A set of example configuration options can be found in the file
[`example.config.json`][json-example], included in this repository.

## Commands

Interacting with FrictionlessBot is done via commands. Commands may be performed by using the prefix 
defined in the configuration and executed in a text channel on a server which the bot is present in.

## Credits

FrictionlessBot and it's constituent components are built primarily using
the serenity-rs framework created by zeyla, which can be found here, [serenity-rs][serenity-rs].

## License

frictionlessbot is distributed under the terms of the GPL-3.

See the license in the github repository for details.

[discord]: https://discordapp.com/
[json-example]: https://github.com/FrictionlessPortals/frictionlessbot/blob/master/res/example.config.json
[my-applications]: https://discordapp.com/developers/applications/me
[new-issue]: https://github.com/FrictionlessPortals/frictionlessbot/issues/new
[rust-lang]: https://www.rust-lang.org/
[serenity-rs]: https://github.com/zeyla/serenity.rs

