use mongodb::bson::Document;
use mongodb::{error::Result, Client, Collection};
use serde_json::Value;

pub async fn get_connection(connection_string: String) -> Client {
    return Client::with_uri_str(connection_string).await.unwrap();
}
pub async fn create(collection: &Collection<Document>, item: Value) -> Result<String> {

    // convert Value to bson document
    let document = bson::to_document(&item).unwrap();

    println!("Inserting document with id");

    let result = collection.insert_one(&document).await?;

    Ok(result.inserted_id.to_string())
}
