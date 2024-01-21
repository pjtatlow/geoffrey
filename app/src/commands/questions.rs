use std::collections::HashMap;

// use crate::modals::find_modal_value;
use crate::{ApplicationContext, Data, Error};
use chrono::Utc;
use entity::{posts, prelude::*};
use poise::serenity_prelude::{
    self as serenity,
    json::{self, Value},
    Channel,
};

use sea_orm::{EntityTrait, NotSet, Set};
// use poise::Modal;
// use sea_orm::ActiveValue::Set;
// use sea_orm::{ActiveModelTrait, EntityTrait};

#[derive(Debug, poise::ChoiceParameter)]
pub enum Tag {
    Linux,
    Python,
    Lists,
    Course,
    Loops,
    Conditionals,
    Functions,
    Dictionaries,
    Grammar,
    Beyond,
    Bio,
    Career,
    Programming,
    Variables,
}

/// Ask an anonymous question
#[poise::command(slash_command, ephemeral)]
pub async fn ask(
    ctx: ApplicationContext<'_>,
    #[description = "Title"] title: String,
    #[description = "Message"] message: String,

    #[description = "Tag"] tag1: Tag,
    #[description = "Tag"] tag2: Option<Tag>,
    #[description = "Tag"] tag3: Option<Tag>,
) -> Result<(), Error> {
    let Data { db } = ctx.data();

    let channel = match ForumChannels::find().one(db).await? {
        Some(c) => c,
        None => {
            ctx.say("I don't know where to ask that question. Please ask your professor to set the forum channel.").await?;
            return Ok(());
        }
    };

    let channel_id: u64 = channel.id.parse()?;

    let channel = match ctx.serenity_context.http.get_channel(channel_id).await? {
        Channel::Guild(c) => c,
        _ => {
            ctx.say("Invalid channel type configured").await?;
            return Ok(());
        }
    };

    let mut selected_tags = vec![tag1.to_string()];
    if let Some(tag2) = tag2 {
        selected_tags.push(tag2.to_string())
    }
    if let Some(tag3) = tag3 {
        selected_tags.push(tag3.to_string())
    }

    let selected_tags: Vec<_> = channel
        .available_tags
        .iter()
        .filter(|x| selected_tags.contains(&x.name))
        .map(|x| Value::Number(x.id.0.into()))
        .collect();

    let thread = channel
        .create_private_thread(&ctx.serenity_context.http, |f| {
            f.kind(serenity::ChannelType::PublicThread).name(title);
            let mut m = HashMap::new();
            m.insert("content", Value::String(message));
            f.0.insert("message", Value::Object(json::hashmap_to_json_map(m)));
            f.0.insert("applied_tags", Value::Array(selected_tags));
            f
        })
        .await?;

    let user_id = ctx.author().id;
    Posts::insert(posts::ActiveModel {
        id: NotSet,
        post_id: Set(thread.id.to_string()),
        user_id: Set(user_id.to_string()),
        date: Set(Utc::now().to_rfc3339()),
        kind: Set(String::from("ANONYMOUS_THREAD")),
        deleted: Set(false),
    })
    .exec(db)
    .await?;

    println!("ADDED ANONYMOUS_THREAD POST");

    // thread
    //     .id
    //     .add_thread_member(&ctx.serenity_context.http, user_id)
    //     .await?;

    // // ctx.serenity_context
    // //     .http
    // //     .create_private_thread(channel_id.0, map)
    // //     .await?;

    // ctx.send(|r| r.ephemeral(true).).await?;
    ctx.say(format!(
        "Your question has been posted. Be sure to follow this thread to see the answer: <#{}> ",
        thread.id.to_string()
    ))
    .await?;
    Ok(())
}
