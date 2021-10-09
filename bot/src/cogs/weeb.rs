use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn anime(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if let Err(e) = msg.reply(ctx, args.rest()).await {
        println!("error in msg: {:?}", e);
    }

    Ok(())
}

#[group]
#[commands(anime)]
pub struct Weeb;
