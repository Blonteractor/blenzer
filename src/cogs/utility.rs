use serenity::builder::CreateMessage;
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    let author = msg.member(ctx).await?;

    let author_roles = match author.roles(ctx).await {
        Some(roles) => roles,
        None => Vec::new(),
    };

    msg.reply(ctx, "as").await?;
    Ok(())
}

#[group]
#[commands(info)]
pub struct Utility;
