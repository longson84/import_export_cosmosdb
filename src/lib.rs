use azure_data_cosmos::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::string::String;

use std::fmt::{Debug, Display, Error, Formatter};

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let collection = get_cosmosdb_collection();
        let test_chat_story = ChatStory {
            id: "02".to_string(),
            prompt: "Generate for me a sample".to_string(),
            master_story: "Here is my master story".to_string(),
            details: "Here are the details".to_string(),
        };

        insert_into_cosmosdb(test_chat_story).await;
        let chat = get_from_cosmosdb(String::from("02"), "Generate for me a sample")
            .await
            .unwrap();

        // assert!(collection.is_some(), "Empty collection");
        assert_eq!(chat.id, "02", "Does not match");
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatStory {
    id: String,
    prompt: String,
    master_story: String,
    details: String,
}

impl CosmosEntity for ChatStory {
    type Entity = String;

    fn partition_key(&self) -> Self::Entity {
        self.prompt.clone()
    }
}

const PRIMARY_KEY: &str =
    "in3Hpy5a4zYg0L6ajN24FuRwPSR8nFBoccrE8OA6O1y9KG4km73dtJBEMPnI4YpVK65e9ul6tVfYACDbxzYmWw==";
const ACCOUNT: &str = "longson";
const DB_NAME: &str = "ChatMaster";
const COLLECTION_NAME: &str = "ChatStories";

fn get_cosmosdb_collection() -> CollectionClient {
    let authorization_token = AuthorizationToken::primary_from_base64(PRIMARY_KEY).unwrap();

    CosmosClient::new(ACCOUNT, authorization_token)
        .database_client(DB_NAME)
        .collection_client(COLLECTION_NAME)
}

async fn insert_into_cosmosdb(chat_story: ChatStory) {
    let collection = get_cosmosdb_collection();

    let _ = collection.create_document(chat_story).is_upsert(true).await;
}
//
async fn get_from_cosmosdb(gid: String, pk: &str) -> Option<ChatStory> {
    let collection = get_cosmosdb_collection();

    // let get_document_response = collection
    //     .document_client(gid.clone(), &gid)
    //     .unwrap()
    //     .get_document::<ChatStory>()
    //     .await
    //     .unwrap();
    //
    // if let GetDocumentResponse::Found(document) = get_document_response {
    //     let q = document.document.document;
    //     return Some(q);
    // } else {
    //     return None;

    match collection
        .document_client(gid.clone(), &pk)
        .unwrap()
        .get_document::<ChatStory>()
        .await
    {
        Ok(GetDocumentResponse::Found(document)) => Some(document.document.document),
        _ => None,
    }
}
// #[tokio::main]
// async fn main() {
//     let test_chat_story = ChatStory {
//         id: "01".to_string(),
//         prompt: "Generate for me a sample".to_string(),
//         master_story: "Here is my master story".to_string(),
//         details: "Here are the details".to_string(),
//     };
//
//     // insert_into_cosmosdb(test_chat_story).await;
//
//     let chat = get_from_cosmosdb(String::from("01"), "Generate for me a sample");
//
//     println!("{:?}", chat.await.unwrap().id);
// }
