use futures_util::StreamExt;
use mongodb::bson::{doc, DateTime};
use mongodb::error::Error as MongoError;
use mongodb::options::{InsertOneOptions, UpdateOptions};
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use serde::{Deserialize, Serialize};
use twilight_model::id::marker::UserMarker;
use twilight_model::id::Id;

use crate::db::get_collection;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageModel {
    #[serde(rename = "_id")]
    pub id: String,
    pub owner_id: Id<UserMarker>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub payload_json: String,
}

impl MessageModel {
    pub async fn create(&self) -> Result<InsertOneResult, MongoError> {
        get_collection::<Self>("messages")
            .insert_one(self, InsertOneOptions::builder().build())
            .await
    }

    pub async fn update(&self) -> Result<UpdateResult, MongoError> {
        get_collection::<Self>("messages")
            .update_one(
                doc! {"_id": &self.id, "owner_id": self.owner_id.to_string()},
                doc! {"$set": {
                    "name": &self.name,
                    "description": self.description.as_deref(),
                    "payload_json": &self.payload_json,
                    "updated_at": self.updated_at,
                }},
                UpdateOptions::builder().build(),
            )
            .await
    }

    pub async fn exists_by_owner_id_and_id(
        user_id: Id<UserMarker>,
        id: &str,
    ) -> Result<bool, MongoError> {
        get_collection::<Self>("messages")
            .count_documents(doc! {"_id": id, "owner_id": user_id.to_string()}, None)
            .await
            .map(|count| count > 0)
    }

    pub async fn find_by_id(id: &str) -> Result<Option<Self>, MongoError> {
        get_collection("messages")
            .find_one(doc! {"_id": id}, None)
            .await
    }

    pub async fn find_by_owner_id_and_id(
        user_id: Id<UserMarker>,
        id: &str,
    ) -> Result<Option<Self>, MongoError> {
        get_collection("messages")
            .find_one(doc! {"_id": id, "owner_id": user_id.to_string()}, None)
            .await
    }

    pub async fn delete_by_owner_id_and_id(
        user_id: Id<UserMarker>,
        id: &str,
    ) -> Result<DeleteResult, MongoError> {
        get_collection::<Self>("messages")
            .delete_one(doc! {"_id": id, "owner_id": user_id.to_string()}, None)
            .await
    }

    pub async fn list_by_owner_id(
        user_id: Id<UserMarker>,
    ) -> Result<Vec<Result<Self, MongoError>>, MongoError> {
        let cursor = get_collection("messages")
            .find(doc! {"owner_id": user_id.to_string()}, None)
            .await?;

        Ok(cursor.collect().await)
    }

    pub async fn count_by_owner_id(user_id: Id<UserMarker>) -> Result<u64, MongoError> {
        get_collection::<Self>("messages")
            .count_documents(doc! {"owner_id": user_id.to_string()}, None)
            .await
    }
}
