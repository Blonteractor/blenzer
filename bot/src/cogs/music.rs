use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let query = args.rest();

    msg.reply(ctx, format!("Query: {}", query)).await?;

    Ok(())
}

#[group]
#[commands(play)]
pub struct Music;
