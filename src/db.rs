use crate::model::UserModel;

use mongodb::bson::{doc, Document};
use mongodb::{options::ClientOptions, Client, Collection};
use mongodb::error::Result;
use serenity::prelude::TypeMapKey;

#[derive(Clone, Debug)]
pub struct DB {
    pub note_collection: Collection<UserModel>,
    pub collection: Collection<Document>,
}

#[serenity::async_trait]
impl TypeMapKey for DB {
    type Value = DB; // Usamos a própria struct DB como valor
}

impl DB {
  pub async fn init() -> Result<Self> {
      let mongodb_uri: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
      let database_name: String =
          std::env::var("MONGO_INITDB_DATABASE").expect("MONGO_INITDB_DATABASE must be set.");
      let mongodb_note_collection: String =
          std::env::var("MONGODB_NOTE_COLLECTION").expect("MONGODB_NOTE_COLLECTION must be set.");

      let mut client_options = ClientOptions::parse(mongodb_uri).await?;
      client_options.app_name = Some(database_name.to_string());

      let client = Client::with_options(client_options)?;
      let database = client.database(database_name.as_str());

      let note_collection = database.collection(mongodb_note_collection.as_str());
      let collection = database.collection::<Document>(mongodb_note_collection.as_str());

      println!("✅ Database connected successfully");

      Ok(Self {
          note_collection,
          collection,
      })
  }

  pub async fn create_user(&self, user_id: &str) -> Result<UserModel> {
      let user = self.note_collection.find_one(doc! {"id": user_id}, None).await?;
      if user.is_none() {
          let user = UserModel {
              id: user_id.to_string(),
              coins: 0,
          };
          self.note_collection.insert_one(user.clone(), None).await?;
          return Ok(user);
      } else {
          return Err(mongodb::error::Error::from(std::io::Error::new(
              std::io::ErrorKind::Other,
              "User already exists",
          )));
      }
  }

  pub async fn get_user(&self, user_id: &str) -> Result<UserModel> {
      let user = self.note_collection.find_one(doc! {"id": user_id}, None).await?;
      if user.is_none() {
          return self.create_user(user_id).await;
      } else {
          return Ok(user.unwrap());
      }
  }
}
