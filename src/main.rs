mod db;
mod model;

use std::env;

use poise::serenity_prelude::*;
use poise::{Framework, FrameworkOptions};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data;

pub mod commands;
pub mod events;

#[tokio::main]
async fn main() {
    db::init().await.expect("Failed to connect to database");

    dotenv::dotenv().expect("Failed to load .env file");  
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(token, intents_config())
        .framework(framework_config())
        .await
        .unwrap();

    if let Err(motivo) = client.start().await {
        println!("Conexão não sucedida\nMotivo: {motivo:?}");
    }
}

fn intents_config() -> GatewayIntents {
    let intents = GatewayIntents::all();

    intents
}

fn framework_config() -> Framework<Data, Error> {
    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: commands::get_commands(),
            event_handler: |ctx, event, framework, data| {
                Box::pin(events::handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    framework
}
