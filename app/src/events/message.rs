use crate::{Data, Error};
use chrono::Utc;
use entity::{posts, prelude::*};
use poise::{
    serenity_prelude::{self as serenity, Channel, Message},
    FrameworkContext,
};
use sea_orm::{EntityTrait, NotSet, Set};

pub async fn handle<'a>(
    ctx: &serenity::Context,
    msg: &Message,
    _framework: FrameworkContext<'a, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    if msg.author.bot {
        return Ok(());
    }

    let Data { db } = data;
    let forum_channel_id = match ForumChannels::find().one(db).await? {
        Some(c) => c.id.parse::<u64>()?,
        None => return Ok(()),
    };

    let channel = match msg.channel(&ctx).await? {
        Channel::Guild(c) => c,
        _ => return Ok(()),
    };

    if let Some(parent_id) = channel.parent_id {
        if parent_id.0 != forum_channel_id {
            // Not the forum channel
            return Ok(());
        }
    } else {
        // Doesn't have a parent, not a message we track
        return Ok(());
    }

    Posts::insert(posts::ActiveModel {
        id: NotSet,
        post_id: Set(msg.id.to_string()),
        user_id: Set(msg.author.id.to_string()),
        date: Set(Utc::now().to_rfc3339()),
        kind: Set(String::from("MESSAGE")),
        deleted: Set(false),
    })
    .exec(db)
    .await?;
    println!("ADDED MESSAGE POST");

    Ok(())
}
