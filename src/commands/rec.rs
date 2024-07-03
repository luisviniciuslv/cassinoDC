use serenity::all::{CommandOptionType, CreateCommandOption};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {

    "Hey, I'm alive!".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("par").description("Gera um número aleatório, caso ele seja par, você ganha, se não, você perde!").add_option(
      CreateCommandOption::new(CommandOptionType::Number, "Aposta", "Valor que deseja apostar")
    )
}
