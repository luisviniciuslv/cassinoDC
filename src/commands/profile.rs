use std::error::Error;

use serenity::framework::standard::macros::command;
use crate::db::get_user;
use serenity::framework::standard::Args;

use serenity::builder::{CreateEmbed, CreateEmbedFooter, CreateMessage};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn profile(ctx: &Context, msg: &Message,  mut args: Args) -> Result<(), Box<dyn Error + Send + Sync>> {
  let mention = args.single::<String>().unwrap_or_default();
  println!("{mention}");
  let user_id;
  let user;
  if mention.is_empty() {
    user_id = msg.author.id.to_string();
    user = msg.author.clone();
  } else {
    user_id = mention.replace("<@", "").replace(">", "").replace("!", "").replace(" ", "");
    user = msg.guild_id.unwrap().member(&ctx.http, UserId::new(user_id.parse::<u64>().unwrap())).await?.user;
  }

  let user_db = get_user(&user_id).await?;
  let footer = CreateEmbedFooter::new("ヾ(￣▽￣)").icon_url(user.face());

  let embed = CreateEmbed::new()
      .title(user.name.clone())
      .description(format!("Perfil do {}", user.name.clone()))
      .field("Coins", user_db.coins.to_string(), true)
      .footer(footer)
      .timestamp(Timestamp::now());

  let builder = CreateMessage::new()
      .embed(embed);

  let msg = msg.channel_id.send_message(&ctx.http, builder).await;
  if let Err(why) = msg {
    println!("Error sending message: {why:?}");
  }
    Ok(())
}
