use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserModel {
  pub _id: String,
  pub coins: i64,
  pub last_reward: i64
}