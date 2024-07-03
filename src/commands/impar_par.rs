use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::db::{get_user, update_coins};
extern crate rand;
use rand::Rng;

// o usuário só pode executar essa chamada a cada 5 minutos
#[command]
pub async fn pi(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
  let escolha = args.single::<String>()?;
  let valor_aposta = args.single::<i32>()?;
  // checar se aposta não é maior doque o valor que usuário tem de coins
  let user = get_user(msg.author.id.to_string().as_str()).await?;

  let mut invalid_input = false;
  let mut message: String = String::new();

  if user.coins < valor_aposta {
    invalid_input = true;
    message = "Você não tem coins suficientes para apostar".to_string();

  } else if valor_aposta <= 0 {
    invalid_input = true;
    message = "Valor de aposta inválido".to_string();
  }

  if invalid_input {
    msg.channel_id.say(&ctx.http, message).await?;
    return Ok(());
  }
  
  let n = generate_random_number(100);

  let ganhou = match escolha.as_str() {
    "p" => n % 2 == 0,
    "i" => n % 2 != 0,
    _ => {
        msg.channel_id
            .say(&ctx.http, "Escolha inválida. Use \"p\" para (par) ou \"i\"  para (impar).")
            .await?;
        return Ok(());
    }
};

  let resultado = if ganhou {
    update_coins(msg.author.id.to_string().as_str(), valor_aposta).await?;
    format!("Número gerado: {n}\nVocê ganhou {valor_aposta} coins")
  } else {
    update_coins(msg.author.id.to_string().as_str(), -valor_aposta).await?;
    format!("Número gerado: {n}\nVocê perdeu {valor_aposta} coins")
  };

  msg.channel_id.say(&ctx.http, resultado).await?;
  Ok(())
}

fn generate_random_number(range: u32) -> u32 {
  let mut rng = rand::thread_rng();
  return rng.gen_range(0..range);
}
