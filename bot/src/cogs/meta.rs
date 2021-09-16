use serenity::framework::standard::{
    macros::{command, group},
    CommandResult,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(e) = msg.reply(ctx, "Pong!").await {
        println!("error in msg: {:?}", e);
    }

    Ok(())
}

#[command]
async fn say(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(e) = msg.reply(ctx, "Yo!").await {
        println!("error in msg: {:?}", e);
    }

    Ok(())
}

#[group]
#[commands(ping, say)]
pub struct Meta;
