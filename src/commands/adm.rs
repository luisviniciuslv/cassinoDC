use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::db::update_coins;

#[command]
#[owners_only]
pub async fn add_coins(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
  let mut user_id = args.single::<String>()?;
  user_id = user_id.replace("<@", "").replace(">", "").replace("!", "").replace(" ", "");
  println!("user_id: {}", user_id);
  let user: Member = msg.guild_id.unwrap().member(&ctx.http, UserId::new(user_id.parse::<u64>().unwrap())).await?;
  let coins = args.single::<String>()?.parse::<i32>()?;

  update_coins(user_id.to_string().as_str(), coins).await?;

  msg.channel_id
      .say(&ctx.http, format!("Coins adicionados com sucesso para o usuário {}", user.mention()))
      .await?;

  Ok(())
}
