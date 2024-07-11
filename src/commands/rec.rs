use poise::CreateReply;

use crate::db::{update_coins, atualize_last_reward};
use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn rec(ctx: Context<'_>) -> Result<(), Error> {
    let user = ctx.author();
    let _ = update_coins(&user.id.to_string(), 100).await.expect("?");
    let _ = atualize_last_reward(&user.id.to_string()).await.expect("?");
    let message = String::from("Você recebeu 100 coins! Use o comando /profile para ver seu saldo.");
    ctx.send(
        CreateReply {
            content: Some(message),
            ..Default::default()
        },
    ).await?;
    Ok(())
}
