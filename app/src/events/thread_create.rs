use crate::{Data, Error};
use chrono::Utc;
use entity::{posts, prelude::*};
use poise::{
    serenity_prelude::{self as serenity, GuildChannel},
    FrameworkContext,
};
use sea_orm::{DatabaseConnection, EntityTrait, NotSet, Set};

pub async fn handle<'a>(
    _ctx: &serenity::Context,
    thread: &GuildChannel,
    framework: FrameworkContext<'a, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    let Data { db } = data;
    let channel = match ForumChannels::find().one(db).await? {
        Some(c) => c,
        // No forum channel registered, no recording of threads!
        None => return Ok(()),
    };
    let forum_channel_id: u64 = channel.id.parse()?;

    add_thread(db, thread, forum_channel_id, framework.bot_id.0).await?;

    Ok(())
}

pub async fn add_thread(
    db: &DatabaseConnection,
    thread: &GuildChannel,
    forum_channel_id: u64,
    bot_id: u64,
) -> Result<(), Error> {
    if let Some(parent_id) = thread.parent_id {
        if parent_id.0 != forum_channel_id {
            // We don't care about this thread creation... only forum threads!
            return Ok(());
        }
    }

    let user_id = match thread.owner_id {
        Some(id) => id,
        None => return Ok(()),
    };
    if user_id == bot_id {
        // No need to log threads created by us
        return Ok(());
    }

    Posts::insert(posts::ActiveModel {
        id: NotSet,
        post_id: Set(thread.id.to_string()),
        user_id: Set(user_id.to_string()),
        date: Set(Utc::now().to_rfc3339()),
        kind: Set(String::from("THREAD")),
        deleted: Set(false),
    })
    .exec(db)
    .await?;

    println!("ADDED THREAD POST");
    Ok(())
}
