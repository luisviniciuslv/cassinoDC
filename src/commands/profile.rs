use poise::{serenity_prelude::{CreateEmbed, CreateEmbedFooter, User, Timestamp}, CreateReply};

use crate::{Context, Error};

use crate::db::get_user;

#[poise::command(slash_command)]
pub async fn profile(ctx: Context<'_>, 
#[description = "Caso queira ver o perfil de algum usuário, mencione-o."]
user: Option<User>) -> Result<(), Error> {
  let user = user.unwrap_or_else(|| ctx.author().clone());

  let user_db = get_user(&user.id.to_string()).await?;
  let footer = CreateEmbedFooter::new("ヾ(￣▽￣)").icon_url(user.face());

  let embed = CreateEmbed::new()
    .title(user.name.clone())
    .description(format!("Perfil do {}", user.name.clone()))
    .field("Coins", user_db.coins.to_string(), true)
    .footer(footer)
    .timestamp(Timestamp::now());

    ctx.send(CreateReply {
      embeds: vec![embed],
      ..Default::default()
  })
  .await
  .unwrap();

  Ok(())
}
