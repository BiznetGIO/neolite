use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;
use serde_json as json;

use crate::client::Client;

pub struct Keypair {
    client: Arc<Client>,
}

impl Keypair {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }
    pub async fn list(&self) -> Result<Vec<KeypairResource>, crate::Error> {
        let response = self.client.get("/keypairs").await?;
        let response: Vec<KeypairResource> = json::from_value(response)?;
        Ok(response)
    }

    // NOTE: The NEOLite REST API doesn't have `get keypair`
    pub async fn get(&self, id: u32) -> Result<KeypairResource, crate::Error> {
        let keys = self.list().await?;
        for key in keys {
            if key.id == id {
                return Ok(key);
            }
        }
        Err(crate::Error::NotFound("Keypair is not found".to_string()))
    }
    pub async fn create(&self, name: &str) -> Result<KeypairResource, crate::Error> {
        let body = json::json!({ "name": name });
        let response = self.client.post("/keypairs", body).await?;
        let response: KeypairResource = json::from_value(response)?;
        Ok(response)
    }
    pub async fn delete(&self, id: u32) -> Result<(), crate::Error> {
        self.client.delete(&format!("/keypairs/{}", id)).await?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeypairResource {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    #[serde(rename = "keypair_id")]
    pub id: u32,
    pub name: String,
    pub public_key: String,
}
