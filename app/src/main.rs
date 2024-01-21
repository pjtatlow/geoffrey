use anyhow::Error;
use migration::{Migrator, MigratorTrait};
use poise::serenity_prelude::{self as serenity, GuildId};
use sea_orm::{Database, DatabaseConnection};

mod commands;
mod dbutil;
mod events;

pub const BIO_165_GUILD: GuildId = GuildId(1043348477419200532);

pub struct Data {
    db: DatabaseConnection,
} // User data, which is stored and accessible in all command invocations
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, Error>;

#[tokio::main]
async fn main() {
    env_logger::init();

    let db: DatabaseConnection = Database::connect("sqlite:test.db")
        .await
        .expect("could not open database");

    Migrator::up(&db, None)
        .await
        .expect("could not run migrations");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::users(), commands::admin(), commands::ask()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(async move { events::handle(ctx, event, framework, data).await })
            },
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("no DISCORD_TOKEN in the environment"))
        .intents(serenity::GatewayIntents::all())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    BIO_165_GUILD,
                )
                .await?;

                Ok(Data { db })
            })
        });

    framework.run().await.unwrap();
}
