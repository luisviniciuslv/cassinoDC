#![allow(deprecated)]
mod commands;
mod db;
mod model;

use std::collections::HashSet;
use std::env;
use std::sync::Arc;

use serenity::all::standard::buckets::LimitedFor;
use serenity::all::standard::macros::hook;
use serenity::all::standard::BucketBuilder;
use serenity::all::CreateInteractionResponse;
use serenity::all::CreateInteractionResponseMessage;
use serenity::all::GuildId;
use serenity::all::Interaction;
use serenity::all::Message;
use serenity::async_trait;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::Configuration;
use serenity::framework::StandardFramework;
use serenity::gateway::ShardManager;
use serenity::http::Http;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use tracing::{error, info};
use serenity::model::id::UserId;
use crate::commands::profile::*;
use crate::commands::adm::*;
use crate::commands::rec::*;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
  type Value = Arc<ShardManager>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
    let cloned_interaction = interaction.clone();
    if let Interaction::Command(command) = interaction {
      let data = command.data.clone();
      let content = match data.name.as_str() {
        "poi" => Some(commands::par_ou_impar::run(cloned_interaction).await),
        _ => Some("not implemented :(".to_string())
      };

      if let Some(content) = content {
        let data = CreateInteractionResponseMessage::new().content(content);
        let builder = CreateInteractionResponse::Message(data);
        if let Err(why) = command.create_response(&ctx.http, builder).await {
          println!("Cannot respond to slash command: {why}");
        }
      }
    }
  }

  async fn ready(&self, ctx: Context, ready: Ready) {
    info!("Connected as {}", ready.user.name);
    
    let guild_id = GuildId::new(1048416271747780650);
    let commands = guild_id.set_commands(&ctx.http, vec![
      commands::par_ou_impar::register()
    ]).await;
  
    println!("I now have the following guild slash commands: {commands:#?}");

    // let guild_command =
    //     Command::create_global_command(&ctx.http, commands::rec::register())
    //         .await;

    // println!("I created the following global slash command: {guild_command:#?}");
  }

  async fn resume(&self, _: Context, _: ResumedEvent) {
    info!("Resumed");
  }
}

#[group]
#[commands(profile, add_coins, rec)]
struct General;

#[tokio::main]
async fn main() {
  db::init().await.expect("Failed to connect to database");

  dotenv::dotenv().expect("Failed to load .env file");

  tracing_subscriber::fmt::init();

  let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

  let http = Http::new(&token);

  let (owners, _bot_id) = match http.get_current_application_info().await {
    Ok(info) => {
      let mut owners = HashSet::new();
      if let Some(owner) = &info.owner {
        owners.insert(owner.id);
        owners.insert(UserId::new(597492835662692371));
      }

      (owners, info.id)
    },
    Err(why) => panic!("Could not access application info: {:?}", why),
  };

  // Create the framework
  let framework = StandardFramework::new().group(&GENERAL_GROUP).bucket("req",
  BucketBuilder::default().limit(1).time_span(300).delay(300)
    .await_ratelimits(0)
    .limit_for(LimitedFor::User)
    .delay_action(|ctx, msg| {
      Box::pin(delay_action(ctx, msg))
    })).await;

  framework.configure(Configuration::new().owners(owners).prefix("!"));

  let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT;

  let mut client = Client::builder(&token, intents)
    .framework(framework)
    .event_handler(Handler)
    .await
    .expect("Err creating client");

  {
    let mut data = client.data.write().await;
    data.insert::<ShardManagerContainer>(client.shard_manager.clone());
  }

  let shard_manager = client.shard_manager.clone();

  tokio::spawn(async move {
    tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
    shard_manager.shutdown_all().await;
  });

  if let Err(why) = client.start().await {
    error!("Client error: {:?}", why);
  }
}

#[hook]
async fn delay_action(ctx: &Context, msg: &Message) {
  // You may want to handle a Discord rate limit if this fails.
  msg.reply(ctx, "Você já recebeu sua recompensa, pode receber apenas a cada 5 minutos!").await.unwrap();
}
