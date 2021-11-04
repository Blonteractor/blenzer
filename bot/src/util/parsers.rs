use serenity::client::Context;
use serenity::framework::standard::{ArgError, Args};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::time::Duration;

pub async fn member<'a>(
    ctx: &Context,
    msg: &Message,
    mut args: Args,
    is_empty_author: bool,
) -> Vec<Result<Member, SerenityError>> {
    let mut results: Vec<Result<Member, SerenityError>> = Vec::new();

    loop {
        match args.single::<UserId>() {
            Ok(id) => results.push(ctx.http.get_member(msg.guild_id.unwrap().0, id.0).await),
            Err(e) => {
                if let ArgError::Parse(_) = e {
                    args.advance();
                }
                if args.is_empty() {
                    args.restore();
                    if args.is_empty() && is_empty_author {
                        results.push(msg.member(ctx).await);
                    }
                    break;
                } else {
                    continue;
                }
            }
        }
    }

    args.restore();
    results
}

pub async fn await_choice_int<'a, S>(
    ctx: &Context,
    msg: &Message,
    max: isize,
    min: isize,
    timeout_seconds: usize,
    on_input_invalid: impl Fn(Option<isize>) -> S,
    on_no_input: impl Fn() -> S,
) -> Result<Option<isize>, SerenityError>
where
    S: ToString,
{
    if let Some(choice_msg) = msg
        .author
        .await_reply(ctx)
        .timeout(Duration::new(timeout_seconds as u64, 0))
        .await
    {
        if let Ok(choice_int) = choice_msg.content.parse::<isize>() {
            if choice_int > min && choice_int <= max {
                choice_msg.delete(ctx).await?;
                Ok(Some(choice_int))
            } else {
                let msg_content = on_input_invalid(Some(choice_int)).to_string();
                msg.channel_id
                    .send_message(ctx, |c| c.content(msg_content))
                    .await?;
                choice_msg.delete(ctx).await?;
                Ok(None)
            }
        } else {
            let msg_content = on_input_invalid(None).to_string();
            msg.channel_id
                .send_message(ctx, |c| c.content(msg_content))
                .await?;
            choice_msg.delete(ctx).await?;
            Ok(None)
        }
    } else {
        let msg_content = on_no_input().to_string();
        msg.channel_id
            .send_message(ctx, |c| c.content(msg_content))
            .await?;
        Ok(None)
    }
}
