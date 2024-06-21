use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::db::DB;

#[command]
pub async fn multiply(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let one = args.single::<f64>()?;
    let two = args.single::<f64>()?;
    let db = DB::init().await?;
    let product = one * two;
    let user = db.get_user(msg.author.id.to_string().as_str()).await?;
    println!("User: {:?}", user);
    msg.channel_id.say(&ctx.http, product.to_string()).await?;

    Ok(())
}
