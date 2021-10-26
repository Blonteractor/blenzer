use serenity::framework::standard::{
    macros::{command, group},
    CommandResult,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn say(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Yo!").await?;

    Ok(())
}

#[group]
#[commands(ping, say)]
pub struct Meta;
