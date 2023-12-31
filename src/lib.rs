use azure_data_cosmos::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::string::String;

use azure_data_cosmos::prelude::IndexingDirective::Default;
use std::fmt::{Debug, Display, Error, Formatter};

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let collection = get_cosmosdb_collection();
        let test_chat_story = ChatStory {
            id: "03".to_string(),
            prompt: "Generate for me a sample".to_string(),
            master_story: "Here is my master story".to_string(),
            details: "Here are the details".to_string(),
        };

        insert_into_cosmosdb(test_chat_story)
            .await
            .expect("Cannot create");
        let chat = get_from_cosmosdb(String::from("04"), "Generate for me a sample")
            .await
            .unwrap();

        assert_eq!(chat.id, "03", "Does not match");
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

const PRIMARY_KEY: &str = "";
const ACCOUNT: &str = "";
const DB_NAME: &str = "ChatMaster";
const COLLECTION_NAME: &str = "ChatStories";

fn get_cosmosdb_collection() -> CollectionClient {
    let authorization_token = match AuthorizationToken::primary_from_base64(PRIMARY_KEY) {
        Ok(token) => token,
        Err(e) => panic!("There was an error {}", e),
    };

    // let authorization_token = AuthorizationToken::primary_from_base64(PRIMARY_KEY).unwrap();

    CosmosClient::new(ACCOUNT, authorization_token)
        .database_client(DB_NAME)
        .collection_client(COLLECTION_NAME)
}

async fn insert_into_cosmosdb(chat_story: ChatStory) -> Result<(), Box<dyn std::error::Error>> {
    let collection = get_cosmosdb_collection();

    let result = collection.create_document(chat_story).is_upsert(true).await;

    match result {
        Ok(_) => Ok(()),
        Err(err) => panic!("Error creating documents {}", err),
    }
}
//
async fn get_from_cosmosdb(gid: String, pk: &str) -> Option<ChatStory> {
    let collection = get_cosmosdb_collection();

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
