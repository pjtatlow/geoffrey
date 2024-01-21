use crate::{Error, BIO_165_GUILD};
use chrono::{FixedOffset, Utc};
use entity::{
    posts,
    prelude::{ForumChannels, Posts},
};
use poise::serenity_prelude::GuildChannel;
use sea_orm::{prelude::DateTimeUtc, DatabaseConnection, EntityTrait, QueryOrder};
use serenity::http::Http;
use std::sync::Arc;

use super::thread_create::add_thread;

pub fn backfill(db: DatabaseConnection, http: Arc<Http>, bot_id: u64) {
    tokio::spawn(async move {
        println!("Starting backfill!");

        match do_backfill(db, http, bot_id).await {
            Ok(_) => println!("Backfill complete!"),
            Err(e) => log::error!("could not perform backfill: {}", e),
        };
    });
}

async fn do_backfill(db: DatabaseConnection, http: Arc<Http>, bot_id: u64) -> Result<(), Error> {
    let channel = match ForumChannels::find().one(&db).await? {
        Some(c) => c,
        // No forum channel registered, no recording of threads!
        None => return Ok(()),
    };
    let forum_channel_id: u64 = channel.id.parse()?;

    let result = Posts::find()
        .order_by_desc(posts::Column::Date)
        .one(&db)
        .await?;
    let latest_date = match result {
        Some(post) => chrono::DateTime::parse_from_rfc3339(&post.date)?,
        None => Utc::now().fixed_offset(),
    }
    .naive_utc();

    let threads = http.get_guild_active_threads(BIO_165_GUILD.0).await?;
    add_threads(
        db,
        http,
        bot_id,
        forum_channel_id,
        &latest_date,
        &threads.threads,
    )
    .await?;
    Ok(())
}

async fn add_threads(
    db: DatabaseConnection,
    http: Arc<Http>,
    bot_id: u64,
    forum_channel_id: u64,
    latest_date: &chrono::NaiveDateTime,
    threads: &Vec<GuildChannel>,
) -> Result<(), Error> {
    for thread in threads {
        if let Some(metadata) = thread.thread_metadata {
            let thread_creation = metadata.create_timestamp.unwrap().naive_utc();
            if &thread_creation > latest_date {
                add_thread(&db, &thread, forum_channel_id, bot_id).await?;
            } else if let Some(archive_timestamp) = &metadata.archive_timestamp {
                let archive_timestamp = archive_timestamp.naive_utc();
                if &archive_timestamp < latest_date {
                    // No need to look at this thread, it was archived before we stopped collecting.
                    continue;
                }
            }

            // TODO: read messages in this thread
            let messages = http.get_messages(thread.id.0, "?limit=100").await?;
        }
    }
    Ok(())
}
