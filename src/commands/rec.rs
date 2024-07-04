use serenity::all::standard::Args;
use serenity::prelude::*;


use serenity::{all::Message, framework::standard::macros::command};
use crate::db::update_coins;

#[command]
#[bucket = "req"]
pub async fn rec(ctx: &Context, msg: &Message, _: Args) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let user_id = msg.author.id.to_string();
    let _ = update_coins(&user_id, 100).await.expect("?");
    msg.channel_id.say(&ctx.http, "Você recebeu 100 coins!").await?;
    Ok(())
}
