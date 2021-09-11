use serenity::framework::standard::{
    macros::{command, group},
    CommandResult,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

use chrono::{DateTime, Utc};

fn date_to_human_readable<'a>(datetime: &'a DateTime<Utc>) -> String {
    let date = datetime.date().to_string();
    format!("{}", date)
}

#[command]
async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    let author = &msg.author;
    let member = &msg.member(ctx).await?;

    let mut author_roles = match member.roles(ctx).await {
        Some(roles) => roles,
        None => Vec::new(),
    };

    author_roles.sort();

    let top_role = author_roles.last().unwrap();
    let avatar_url = &author.avatar_url().unwrap_or(String::new());
    msg.channel_id
        .send_message(ctx, |c| {
            c.reference_message(msg).embed(|e| {
                e.thumbnail(avatar_url)
                    .color(top_role.colour)
                    .url(avatar_url)
                    .title(format!(
                        "{name}#{discrim}",
                        name = author.name,
                        discrim = author.discriminator
                    ))
                    .field("ID", author.id, true)
                    .field(
                        "Nickname",
                        member.nick.as_ref().unwrap_or(&"NA".to_string()),
                        true,
                    )
                    .field("Top Role", top_role.mention(), true)
                    .field(
                        "Created At",
                        date_to_human_readable(&author.created_at()),
                        true,
                    )
                    .field(
                        "Joined At",
                        date_to_human_readable(&member.joined_at.unwrap()),
                        true,
                    )
                    .field(
                        format!("Roles({})", author_roles.len()),
                        author_roles
                            .iter()
                            .rev()
                            .map(|r| r.mention().to_string())
                            .collect::<Vec<String>>()
                            .join("\n"),
                        false,
                    )
            })
        })
        .await?;
    Ok(())
}

#[group]
#[commands(info)]
pub struct Utility;
