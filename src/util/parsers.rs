use serenity::client::Context;
use serenity::framework::standard::Args;
use serenity::model::prelude::*;
use serenity::prelude::*;

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
            Err(_) => {
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
