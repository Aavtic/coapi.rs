extern crate mongodb;

use crate::axum_serve::{DBAddQuestion, ExpectedInputOutput};

use mongodb::bson::{doc, Document};
use mongodb::Client;
use mongodb::options::ClientOptions;
use serde::{Serialize, Deserialize};

use futures::stream::StreamExt;

trait DbStruct {
    fn get_uuid(&self) -> String;
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DbAddQuestion {
    pub title: String,
    pub description: String,
    pub data: Vec<ExpectedInputOutput>,
    pub uuid: String,
    pub code_template: String,
}

impl DbStruct for DbAddQuestion {
    fn get_uuid(&self) -> String {
        self.uuid.to_string()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct PersonDetails {
    name: String,
    age: u16,
}

pub async fn get_question(client: &Client, db_name: &str, coll_name: &str, questionid: String) -> Option<DbAddQuestion> {
    let collection = client.database(db_name).collection::<DbAddQuestion>(coll_name);
    let result = collection.find_one(doc! {"uuid": questionid}).await.unwrap();

    if let Some(res) = result {
        return Some(res)
    } else {
        return None
    }
}

pub async fn get_all_questions(client: &Client, db_name: &str, coll_name: &str) -> Vec<DbAddQuestion>{
    let collection = client.database(db_name).collection::<DbAddQuestion>(coll_name);
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

pub async fn insert_document(client: &Client, db_name: &str, coll_name: &str, doc: &DBAddQuestion) -> DbAddQuestion {
    let coll = client.database(db_name).collection::<DbAddQuestion>(coll_name);
    let doc_data = &doc.data;
    let db_update = DbAddQuestion {
        title: doc.title.clone(),
        description: doc.description.clone(),
        data: doc_data.to_vec(),
        uuid: doc.uuid.clone(),
        code_template: doc.template_code.to_string(),
    };

    coll.insert_one(db_update.clone()).await.unwrap();
    return db_update;
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
