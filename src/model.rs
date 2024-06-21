use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserModel {
    #[serde(rename = "_id")]
    pub id: String,
    pub coins: i32
}