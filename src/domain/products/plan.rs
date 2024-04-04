use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json as json;

use super::{ip::Ip, os::Os};
use crate::client::Client;

#[derive(Debug)]
pub struct Plan {
    client: Arc<Client>,
}

impl Plan {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }
    pub async fn list_vm(&self) -> Result<Vec<PlanResource>, crate::Error> {
        let response = self.client.get("/products").await?;
        let response: Vec<PlanResource> = json::from_value(response)?;
        Ok(response)
    }
    pub async fn get_vm(&self, id: u32) -> Result<PlanResource, crate::Error> {
        let response = self.client.get(&format!("/products/{id}")).await?;
        let response: PlanResource = json::from_value(response)?;
        let response = response.with_client(Arc::clone(&self.client));
        Ok(response)
    }
    //
    // Snapshots
    //
    pub async fn list_snapshot(&self) -> Result<Vec<PlanResource>, crate::Error> {
        let response = self.client.get("/snapshots/products").await?;
        let response: Vec<PlanResource> = json::from_value(response)?;
        Ok(response)
    }
    pub async fn get_snapshot(&self, id: u32) -> Result<PlanResource, crate::Error> {
        let response = self
            .client
            .get(&format!("/snapshots/products/{id}"))
            .await?;
        let response: PlanResource = json::from_value(response)?;
        Ok(response)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlanResource {
    #[serde(rename = "product_id")]
    pub id: u32,
    pub name: String,
    pub description: String,
    pub category_id: u32,
    pub category_name: String,
    pub options: Options,
    pub billing: Vec<Billing>,
    #[serde(skip)]
    client: Arc<Client>,
}

impl PlanResource {
    fn with_client(mut self, client: Arc<Client>) -> Self {
        self.client = client;
        self
    }
    pub async fn os(&self) -> Result<Os, crate::Error> {
        let os = Os::new(Arc::clone(&self.client), self.id);
        Ok(os)
    }
    pub async fn ip(&self) -> Result<Ip, crate::Error> {
        let ip = Ip::new(Arc::clone(&self.client), self.id);
        Ok(ip)
    }
    pub async fn get_billing(&self, label: &str) -> Result<Billing, crate::Error> {
        for billing in &self.billing {
            if billing.label == label {
                return Ok(billing.clone());
            }
        }
        Err(crate::Error::NotFound("Billing is not found".to_string()))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Billing {
    pub label: String,
    pub cycle: String,
    pub price: u32,
    pub components: Option<Vec<Component>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Component {
    pub label: String,
    pub field: String,
    pub prices: Vec<Price>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Price {
    pub qty_min: i32,
    pub qty_max: i32,
    pub price: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Options {
    #[serde(rename = "type")]
    pub r#type: String,
    pub cores: u32,
    pub memory: u32,
    pub allow_downgrade: i32,
}
