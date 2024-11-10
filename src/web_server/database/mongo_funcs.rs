extern crate mongodb;

use crate::axum_serve::AddQuestion;

use mongodb::bson::{doc, Document};
use mongodb::Client;
use mongodb::options::ClientOptions;
use serde::{Serialize, Deserialize};
use futures::StreamExt;


#[derive(Serialize, Deserialize, Clone, Debug)]
struct PersonDetails {
    name: String,
    age: u16,
}

pub async fn get_all_questions(client: &Client, db_name: &str, coll_name: &str) -> Vec<AddQuestion>{
    let collection = client.database(db_name).collection::<AddQuestion>(coll_name);
    let mut result = collection.find(doc! {}).await.unwrap();
    let mut questions = Vec::new();

    while let Some(doc) = result.next().await {
        questions.push(doc.unwrap())
    }
    return questions;
}

pub async fn create_collection(client: &Client, db_name: &str, coll_name: &str) {
    let db = client.database(db_name);
    db.create_collection(coll_name).await.unwrap();
}

pub async fn insert_document(client: &Client, db_name: &str, coll_name: &str, doc: &AddQuestion) {
    let coll = client.database(db_name).collection::<AddQuestion>(coll_name);

    coll.insert_one(doc).await.unwrap();
}

pub async fn find_document(client: &Client, db_name: &str, coll_name: &str, filter: Document) {
    let db = client.database(db_name);
    let coll = db.collection::<PersonDetails>(coll_name);

    let result = coll.find_one(filter.clone()).await.unwrap();
    match result {
        Some(doc) => println!("Found Document!\n{:?}", doc),
        None => println!("Could not find document!"),
    }
}

pub async fn delete_document(client: &Client, db_name: &str, coll_name: &str, filter: Document) {
    let collection = client.database(db_name).collection::<PersonDetails>(coll_name);
    collection.delete_one(filter.clone()).await.unwrap();
}

pub async fn update_document(client: &Client, db_name: &str, coll_name: &str, filter: Document, update: Document) {
    let collection = client.database(db_name).collection::<PersonDetails>(coll_name);
    collection.update_one(filter, update).await.unwrap();
}

pub async fn connect(mongo_addr: &str) -> Client {
    let client_options = ClientOptions::parse(mongo_addr).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    return client;
}
