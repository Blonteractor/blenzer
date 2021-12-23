extern crate dotenv;

pub mod cogs;
pub mod util;

use std::collections::HashSet;
use std::env;

use dotenv::dotenv;

use log::{debug, error, info};

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::CommandError;
use serenity::framework::standard::{
    help_commands,
    macros::{help, hook},
    Args, CommandGroup, CommandResult, DispatchError, HelpOptions, StandardFramework,
};

use serenity::model::{
    channel::Message,
    prelude::{Ready, UserId},
};

use songbird::SerenityInit;

use cogs::meta::*;
use cogs::music::*;
use cogs::utility::*;
use cogs::weeb::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, data_about_bot: Ready) {
        info!("Client connected as {}", data_about_bot.user.name);
    }
}

#[help]
async fn my_help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[hook]
async fn unrecognised_command_hook(_: &Context, msg: &Message, unrecognised_command_name: &str) {
    debug!(
        "User {:?} tried to execute the command {:?} which doesnt exist",
        msg.author.name, unrecognised_command_name
    );
}

#[hook]
async fn command_error_hook(_: &Context, _: &Message, error: DispatchError) {
    error!("Error occured in command: {:?}", error)
}

#[hook]
async fn after_hook(_: &Context, _: &Message, cmd_name: &str, error: Result<(), CommandError>) {
    //  Print out an error if it happened
    if let Err(why) = error {
        error!("Error in {}: {:?}", cmd_name, why);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").unwrap();
    let bot_prefix = env::var("BOT_PREFIX").unwrap();

    let application_id = env::var("DISCORD_APPLICATION_ID")
        .unwrap()
        .parse::<u64>()
        .unwrap();

    //# Build the framework (setting prefix, command hooks, etc)
    let framework = StandardFramework::new()
        .configure(|c| c.prefix(bot_prefix))
        .unrecognised_command(unrecognised_command_hook)
        .on_dispatch_error(command_error_hook)
        .after(after_hook)
        .help(&MY_HELP)
        .group(&META_GROUP)
        .group(&UTILITY_GROUP)
        .group(&MUSIC_GROUP)
        .group(&WEEB_GROUP);

    //# Build the client using the framework and the token
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .application_id(application_id)
        .register_songbird()
        .await
        .expect("Error creating client.");

    // Start listening for events by starting a single shard
    if let Err(e) = client
        .start()
        .await
        .map_err(|why| error!("Client ended: {:?}", why))
    {
        error!("Error in starting client: {:?}", e);
    }
}
