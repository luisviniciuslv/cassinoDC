use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::db::get_user;

#[command]
pub async fn hello(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user = get_user(msg.author.id.to_string().as_str()).await?;
    println!("User: {:?}", user);
    let one = args.single::<f64>()?;
    println!("{}", one);
    msg.channel_id.say(&ctx.http, "test".to_string()).await?;

    Ok(())
}
