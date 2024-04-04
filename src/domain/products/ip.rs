use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json as json;

use crate::client::Client;

#[derive(Debug, Default)]
pub struct Ip {
    pub product_id: u32,
    client: Arc<Client>,
}

impl Ip {
    pub fn new(client: Arc<Client>, product_id: u32) -> Self {
        Self {
            client,
            product_id: product_id.to_owned(),
        }
    }
    pub async fn is_available(&self) -> Result<bool, crate::Error> {
        let response = self
            .client
            .get(&format!("/products/{}/ip-availability", self.product_id))
            .await?;
        let response: IpResource = json::from_value(response)?;
        Ok(response.available)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpResource {
    pub available: bool,
}
