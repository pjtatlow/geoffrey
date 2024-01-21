use crate::{Context, Data, Error};
use entity::{prelude::*, users};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait};

#[poise::command(slash_command, subcommands("register", "whoami"))]
pub async fn users(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Registers your Discord user with the course.
#[poise::command(slash_command, ephemeral)]
pub async fn register(
    ctx: Context<'_>,
    #[description = "Your full name"] name: String,
    #[description = "Your BYU NetID (e.g. cosmoc)"] net_id: String,
) -> Result<(), Error> {
    let Data { db } = ctx.data();
    let user_id = ctx.author().id.0.to_string();

    let existing_user = Users::find_by_id(&user_id).one(db).await?;
    match existing_user {
        Some(u) => {
            let mut u: users::ActiveModel = u.into();
            u.name = Set(name);
            u.net_id = Set(net_id);
            u.update(db).await?;
        }
        None => {
            let u = users::ActiveModel {
                id: Set(user_id),
                net_id: Set(net_id),
                name: Set(name),
            };
            u.insert(db).await?;
        }
    }

    ctx.say("Registration complete.").await?;

    Ok(())
}

/// Check to see if Geoffrey knows who you are.
#[poise::command(slash_command, ephemeral)]
pub async fn whoami(ctx: Context<'_>) -> Result<(), Error> {
    let Data { db } = ctx.data();
    let user_id = ctx.author().id.0.to_string();

    let existing_user = Users::find_by_id(&user_id).one(db).await?;

    let message: String = match existing_user {
        Some(u) => format!("You are {} and your NetID is {}", u.name, u.net_id),
        None => "We have never had the pleasure to meet.\n\n(try using /register to make Geoffrey remember you)".to_string()
    };
    ctx.say(message).await?;

    Ok(())
}
