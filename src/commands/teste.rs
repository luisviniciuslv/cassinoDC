use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::db::{get_user, update_coins};
extern crate rand;
use rand::Rng;

// o usuário só pode executar essa chamada a cada 5 minutos
#[command]
#[bucket="req"]
pub async fn par(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let aposta = args.single::<i32>()?;
    // checar se aposta não é maior doque o valor que usuário tem de coins
    let user = get_user(msg.author.id.to_string().as_str()).await?;

    if user.coins < aposta {
        msg.channel_id.say(&ctx.http, "Você não tem coins suficientes para apostar").await?;
        return Ok(());
    }

    let n = generate_random_number();
    if n % 2 == 0 {
        update_coins(msg.author.id.to_string().as_str(), aposta).await?;
        msg.channel_id.say(&ctx.http, format!("Número gerado: {n}\nVocê ganhou {aposta} coins")).await?;
    } else {
        update_coins(msg.author.id.to_string().as_str(), -aposta).await?;
        msg.channel_id.say(&ctx.http, format!("Número gerado: {n}\nVocê perdeu {aposta} coins")).await?;
    }

    Ok(())
}

fn generate_random_number() -> u32 {
    let mut rng = rand::thread_rng();
    return  rng.gen_range(0..100);
}
