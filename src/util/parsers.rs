use serenity::client::Context;
use serenity::framework::standard::Args;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn member<'a>(
    ctx: &Context,
    msg: &Message,
    mut args: Args,
) -> Result<Member, SerenityError> {
    match args.single::<UserId>() {
        Ok(id) => ctx.http.get_member(msg.guild_id.unwrap().0, id.0).await,
        Err(_) => {
            if args.is_empty() {
                msg.member(ctx).await
            } else {
                msg.reply(
                    ctx,
                    format!(
                        "User ***{}*** not found, please tag the user or use the ID.",
                        args.single::<String>()
                            .unwrap_or("***Invalid Input***".to_string())
                    ),
                )
                .await?;
                return Err(SerenityError::Other(
                    "Couldn't parse the member out of the args.",
                ));
            }
        }
    }
}
