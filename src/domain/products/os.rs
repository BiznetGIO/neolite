use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json as json;

use crate::client::Client;

#[derive(Debug, Default)]
pub struct Os {
    client: Arc<Client>,
    pub product_id: u32,
}

impl Os {
    pub fn new(client: Arc<Client>, product_id: u32) -> Self {
        Self { client, product_id }
    }
    pub async fn list(&self) -> Result<Vec<OsResource>, crate::Error> {
        let response = self
            .client
            .get(&format!("/products/{}/oss", self.product_id))
            .await?;
        let response: Vec<OsResource> = json::from_value(response)?;
        Ok(response)
    }
    // NOTE: The NEOLite REST API doesn't have `get OS`
    pub async fn get(&self, id: u32) -> Result<OsResource, crate::Error> {
        let oses = self.list().await?;
        for os in oses {
            if os.id == id {
                return Ok(os);
            }
        }
        Err(crate::Error::NotFound("OS is not found".to_string()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsResource {
    #[serde(rename = "vmid")]
    pub id: u32,
    pub node: String,
    pub name: String,
    pub maxmem: u64,
    pub maxcpu: u32,
}
