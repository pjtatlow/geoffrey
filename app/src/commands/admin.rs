use crate::{Context, Data, Error};
use anyhow::bail;
use entity::{courses, forum_channels, prelude::*};
use poise::serenity_prelude as serenity;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, NotSet, QueryFilter, Set};

#[poise::command(
    slash_command,
    subcommands("add_course", "remove_course", "set_forum"),
    required_permissions = "ADMINISTRATOR",
    ephemeral
)]
pub async fn admin(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Tell Geoffrey about a new course.
#[poise::command(slash_command, ephemeral)]
pub async fn add_course(
    ctx: Context<'_>,
    #[description = "Unique course name"] name: String,
    #[description = "Start date (YYYY/MM/DD)"] start_date: String,
    #[description = "End date (YYYY/MM/DD)"] end_date: String,
) -> Result<(), Error> {
    let Data { db } = ctx.data();

    if Courses::find()
        .filter(courses::Column::Name.eq(&name))
        .one(db)
        .await?
        .is_some()
    {
        bail!("Course already exists")
    }

    match chrono::NaiveDate::parse_from_str(&start_date, "%Y/%m/%d") {
        Ok(_) => (),
        Err(e) => {
            bail!("invalid start date: {}", e)
        }
    }

    match chrono::NaiveDate::parse_from_str(&end_date, "%Y/%m/%d") {
        Ok(_) => (),
        Err(e) => {
            bail!("invalid end date: {}", e)
        }
    }

    let new_course = courses::ActiveModel {
        id: NotSet,
        name: Set(name),
        start: Set(start_date),
        end: Set(end_date),
    };

    new_course.insert(db).await?;

    ctx.say("Course added.").await?;
    Ok(())
}

async fn autocomplete_name<'a>(ctx: Context<'_>, partial: &'a str) -> Vec<String> {
    let Data { db } = ctx.data();
    let mut names = Vec::new();
    let mut q = Courses::find();
    if partial.len() > 0 {
        q = q.filter(courses::Column::Name.like(format!("%{}%", partial)));
    }

    match q.all(db).await {
        Ok(courses) => {
            for course in courses {
                names.push(course.name);
            }
        }
        Err(e) => {
            log::error!("unable to autocomplete courses: {}", e)
        }
    }

    names
}

/// Tell Geoffrey to forget about a course.
#[poise::command(slash_command, ephemeral)]
pub async fn remove_course(
    ctx: Context<'_>,
    #[description = "Who to greet"]
    #[autocomplete = "autocomplete_name"]
    name: String,
) -> Result<(), Error> {
    let Data { db } = ctx.data();
    match Courses::find()
        .filter(courses::Column::Name.eq(&name))
        .one(db)
        .await?
    {
        Some(course) => {
            course.delete(db).await?;
        }
        None => {
            bail!("Course not found")
        }
    }

    ctx.say("Course removed").await?;
    Ok(())
}

/// Tell Geoffrey where anonymous questions go.
#[poise::command(slash_command, ephemeral)]
pub async fn set_forum(
    ctx: Context<'_>,
    #[channel_types("Forum")] channel: serenity::GuildChannel,
) -> Result<(), Error> {
    let Data { db } = ctx.data();
    ForumChannels::delete_many().exec(db).await?;

    ForumChannels::insert(forum_channels::ActiveModel {
        id: Set(channel.id.to_string()),
    })
    .exec(db)
    .await?;

    ctx.say("Forum channel set").await?;
    Ok(())
}
