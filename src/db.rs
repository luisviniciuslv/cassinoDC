use crate::model::UserModel;

use lazy_static::lazy_static;
use mongodb::bson::{doc, Document};
use mongodb::{options::ClientOptions, Client, Collection};
use mongodb::error::Result;
use std::sync::Mutex;
use mongodb::bson::to_bson;
use mongodb::bson::from_document; // Add this import


lazy_static! {
    static ref DB_CONNECTION: Mutex<Option<Client>> = Mutex::new(None);
    static ref USER_COLLECTION: Mutex<Option<Collection<Document>>> = Mutex::new(None);
}

pub async fn init() -> Result<()> {
    dotenv::dotenv().expect("Failed to load .env file");

    let mongodb_uri: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let database_name: String =
        std::env::var("MONGO_INITDB_DATABASE").expect("MONGO_INITDB_DATABASE must be set.");
    let user_collection_name: String =
        std::env::var("MONGODB_USERS_COLLECTION").expect("MONGODB_USERS_COLLECTION must be set.");

    let mut client_options = ClientOptions::parse(mongodb_uri).await?;
    client_options.app_name = Some(database_name.to_string());

    let client = Client::with_options(client_options)?;
    let database = client.database(&database_name);

    *DB_CONNECTION.lock().unwrap() = Some(client);
    *USER_COLLECTION.lock().unwrap() = Some(database.collection::<Document>(&user_collection_name));

    println!("✅ Database connected successfully");
    Ok(())
}

pub async fn create_user(user_id: &str) -> Result<UserModel> {
    let user_collection: Collection<Document> = USER_COLLECTION.lock().unwrap().clone().unwrap();
    let user = user_collection.find_one(doc! {"_id": user_id}, None).await?;
    if user.is_none() {
        let user = UserModel {
            id: user_id.to_string(),
            coins: 0,
        };
        
        let bson_user = to_bson(&user)?; 
        let document: mongodb::bson::Document = bson_user.as_document().unwrap().clone();
        user_collection.insert_one(document, None).await?;            
        return Ok(user);
    } else {
        return Err(mongodb::error::Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "User already exists",
        )));
    }
}

pub async fn get_user(user_id: &str) -> Result<UserModel> {
    let user_collection: Collection<Document> = USER_COLLECTION.lock().unwrap().clone().unwrap();

    let user_doc = user_collection.find_one(doc! {"_id": user_id}, None).await?;
    if let Some(doc) = user_doc {
        // Convert the Document back to UserModel
        let user: UserModel = from_document(doc)?; 
        Ok(user)
    } else {
        create_user(user_id).await
    }
}

pub async fn update_user(user_id: &str, field: &str, value: i32) -> Result<UserModel> {
    let user_collection: Collection<Document> = USER_COLLECTION.lock().unwrap().clone().unwrap();
    let user = get_user(user_id).await?;
    let updated_user = UserModel {
        id: user_id.to_string(),
        coins: value,
    };

    user_collection.update_one(
        doc! {"_id": user_id},
        doc! {"$set": {field: value}},
        None,
    ).await?;

    Ok(updated_user)
}
