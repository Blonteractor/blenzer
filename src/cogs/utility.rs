use super::super::util;
use chrono::{DateTime, Datelike, Month, Utc};
use num_traits::cast::FromPrimitive;
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

fn date_to_human_readable<'a>(datetime: &'a DateTime<Utc>) -> String {
    let date = datetime.date();

    let day = date.weekday();
    let month = Month::from_u32(date.month()).unwrap().name();
    let year = date.year();

    let mut date_str = date.to_string();

    let date_filtered = date_str.split("-").nth(1).unwrap();

    let suffix = match date_filtered.parse().unwrap() {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };

    date_str = format!("{}{}", date_filtered, suffix);

    format!(
        "{day}, {date} {month} {year}",
        day = day,
        date = date_str,
        month = month,
        year = year,
    )
}

#[command]
async fn info(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let members = util::parsers::member(ctx, msg, args, true).await;

    for result in members {
        let member = match result {
            Ok(m) => m,
            Err(_) => continue,
        };

        let mut member_roles = match member.roles(ctx).await {
            Some(roles) => roles,
            None => Vec::new(),
        };

        member_roles.sort();

        let top_role = member_roles.last().unwrap();
        let avatar_url = &member.user.avatar_url().unwrap_or(String::new());
        msg.channel_id
            .send_message(ctx, |c| {
                c.reference_message(msg).embed(|e| {
                    e.thumbnail(avatar_url)
                        .color(top_role.colour)
                        .url(avatar_url)
                        .title(format!(
                            "{name}#{discrim}",
                            name = member.user.name,
                            discrim = member.user.discriminator
                        ))
                        .field("ID", member.user.id, true)
                        .field(
                            "Nickname",
                            member.nick.as_ref().unwrap_or(&"NA".to_string()),
                            true,
                        )
                        .field("Top Role", top_role.mention(), true)
                        .field(
                            "Created At",
                            date_to_human_readable(&member.user.created_at()),
                            true,
                        )
                        .field(
                            "Joined At",
                            date_to_human_readable(&member.joined_at.unwrap()),
                            true,
                        )
                        .field(
                            format!("Roles({})", member_roles.len()),
                            member_roles
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
    }

    Ok(())
}

#[group]
#[commands(info)]
pub struct Utility;
