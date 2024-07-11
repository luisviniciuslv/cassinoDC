use crate::Error;
use poise::serenity_prelude::ActivityData;
use poise::serenity_prelude::Context;
use poise::serenity_prelude::Ready;

pub async fn ready(event: &Ready, ctx: &Context) -> Result<(), Error> {
    let bot = ctx.http.get_current_user().await.unwrap();

    println!(
        "[{}] Está online e pronto para operar
        e está presente em {} servidor(es)!",
        bot.name,
        event.guilds.len()
    );

    ctx.set_presence(
        Some(ActivityData::playing("apenas.")),
        poise::serenity_prelude::OnlineStatus::DoNotDisturb,
    );

    Ok(())
}
