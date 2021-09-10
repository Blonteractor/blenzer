extern crate dotenv;
mod cogs;

use std::collections::HashSet;
use std::env;

use dotenv::dotenv;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    help_commands,
    macros::{help, hook},
    Args, CommandGroup, CommandResult, DispatchError, HelpOptions, StandardFramework,
};
use serenity::model::{
    channel::Message,
    prelude::{Ready, UserId},
};

use cogs::meta::*;
use cogs::utility::*;

const BOT_PREFIX: &str = "bl ";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, data_about_bot: Ready) {
        println!("Client connected as {}", data_about_bot.user.name);
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
    println!(
        "User {:?} tried to execute the command {:?} which doesnt exist",
        msg.author.name, unrecognised_command_name
    );
}

#[hook]
async fn command_error_hook(_: &Context, msg: &Message, error: DispatchError) {
    match error {
        _ => eprintln!("Error occured in command: {:?}", error),
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").unwrap();

    //# Build the framework (setting prefix, command hooks, etc)
    let framework = StandardFramework::new()
        .configure(|c| c.prefix(BOT_PREFIX))
        .unrecognised_command(unrecognised_command_hook)
        .on_dispatch_error(command_error_hook)
        .help(&MY_HELP)
        .group(&META_GROUP)
        .group(&UTILITY_GROUP);

    //# Build the client using the framework and the token
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client.");

    // Start listening for events by starting a single shard
    if let Err(e) = client.start().await {
        println!("An error occured while running the client: {:?}", e);
    }
}
