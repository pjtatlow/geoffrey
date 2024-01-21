use crate::{Data, Error};
use poise::{serenity_prelude as serenity, Event, FrameworkContext};

mod backfill;
mod message;
mod thread_create;

use backfill::backfill;

pub async fn handle<'a>(
    ctx: &serenity::Context,
    event: &Event<'a>,
    framework: FrameworkContext<'a, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        Event::Message { new_message } => message::handle(ctx, new_message, framework, data).await,
        Event::Ready { data_about_bot } => {
            println!("Geoffrey is ready");
            backfill(data.db.clone(), ctx.http.clone(), data_about_bot.user.id.0);
            Ok(())
        }
        Event::ThreadCreate { thread } => thread_create::handle(ctx, thread, framework, data).await,
        _ => Ok(()),
    }
}
