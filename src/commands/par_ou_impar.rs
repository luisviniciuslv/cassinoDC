use rand::Rng;
use serenity::all::{CommandOptionType, CreateCommandOption};
use serenity::builder::CreateCommand;
use serenity::all::Interaction;
use crate::db::{get_user, update_coins};

pub async fn run(interaction: Interaction) -> String {

  let options;
  let user_id;
  if let Interaction::Command(command) = interaction {
    options = command.data.options;
    user_id = command.user.id;
  } else { return "not implemented :(".to_string(); }

  let option = options.get(0).unwrap().value.as_str().expect("Opção inválida");
  let valor_aposta = options.get(1).unwrap().value.as_i64().expect("Valor de aposta inválido");

  let user = get_user(&user_id.to_string()).await.unwrap();

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
    return message;
  }

  let n = generate_random_number(100);

  let ganhou = match option {
    "par" => n % 2 == 0,
    "impar" => n % 2 != 0,
    _ => {
        return "Escolha inválida. Use \"p\" para (par) ou \"i\"  para (impar).".to_owned()
    }
};

  let resultado = if ganhou {
      let _ = update_coins(&user_id.to_string(), valor_aposta).await.expect("?");
      format!("Número gerado: {n}\nVocê ganhou {valor_aposta} coins")
  } else {
      let _ = update_coins(&user_id.to_string(), -valor_aposta).await.expect("?");
      format!("Número gerado: {n}\nVocê perdeu {valor_aposta} coins")
  };
  resultado
}

pub fn register() -> CreateCommand {
    CreateCommand::new("poi").description("Gera um número aleatório, se for da sua escolha você ganha.")
    .add_option(
      CreateCommandOption::new(CommandOptionType::String, "escolha", "Deseja apostar no par ou no impar?")
      .add_string_choice("par", "par")
      .add_string_choice("impar", "impar"))
    .add_option(
      CreateCommandOption::new(CommandOptionType::Integer, "aposta", "Valor que deseja apostar")
    )
}

fn generate_random_number(range: u32) -> u32 {
  let mut rng = rand::thread_rng();
  return rng.gen_range(0..range);
}
