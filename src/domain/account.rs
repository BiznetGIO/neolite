use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;
use serde_json as json;

use crate::client::Client;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AccountStatus {
    Active,
    Pending,
    Suspended,
    Terminated,
}

impl AccountStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "Active",
            Self::Pending => "Pending",
            Self::Suspended => "Suspended",
            Self::Terminated => "Terminated",
        }
    }
}

pub struct Account {
    client: Arc<Client>,
}

impl Account {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }
    pub async fn list(&self) -> Result<Vec<AccountResource>, crate::Error> {
        let response = self.client.get("/accounts").await?;
        let response: Vec<AccountResource> = json::from_value(response)?;
        Ok(response)
    }
    pub async fn list_with_status(
        &self,
        status: AccountStatus,
    ) -> Result<Vec<AccountResource>, crate::Error> {
        let response = self
            .client
            .get(&format!("/accounts?status={}", status.as_str()))
            .await?;
        let response: Vec<AccountResource> = json::from_value(response)?;
        Ok(response)
    }
    // pub async fn get(&self, id: u32) -> Result<AccountResource, crate::Error> {
    //     let response = self.client.get(&format!("/accounts/{id}")).await;
    //     match response {
    //         Ok(response) => {
    //             let response: AccountResource = json::from_value(response)?;
    //             Ok(response)
    //         }
    //         Err(e) => match e {
    //             crate::Error::NotFound(_) => {
    //                 Err(crate::Error::NotFound("Account not found".into()))
    //             }
    //             _ => Err(e),
    //         },
    //     }
    // }

    //
    // Snapshots
    //
    pub async fn list_snapshot(&self) -> Result<Vec<SnapshotAccountResource>, crate::Error> {
        let response = self.client.get("/snapshots/accounts").await?;
        let response: Vec<SnapshotAccountResource> = json::from_value(response)?;
        Ok(response)
    }

    pub async fn get_snapshot(&self, id: u32) -> Result<SnapshotAccountResource, crate::Error> {
        let response = self.client.get(&format!("/snapshots/accounts/{id}")).await;
        match response {
            Ok(response) => {
                let response: SnapshotAccountResource = json::from_value(response)?;
                Ok(response)
            }
            Err(e) => match e {
                crate::Error::NotFound(_) => {
                    Err(crate::Error::NotFound("Snapshot not found".into()))
                }
                _ => Err(e),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountResource {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    #[serde(rename = "account_id")]
    pub id: u32,
    pub domain: String,
    pub status: AccountStatus,
    pub billingcycle: String,
    pub date_created: String,
    pub next_due: String,
    pub recurring_amount: i32,
    pub extra_details: ExtraDetails,
    pub product_id: u32,
    pub product_name: String,
    pub description: String,
    pub category_id: u32,
    pub category_name: String,
    pub last_invoice: LastInvoice,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LastInvoice {
    pub id: u32,
    pub paid_id: u32,
    pub status: String,
    pub date: String,
    pub duedate: String,
    pub paybefore: String,
    pub datepaid: String,
    pub invoice_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraDetails {
    pub region: String,
    pub region_label: String,
    pub description: String,
    pub name: String,
    pub tenant_id: Option<String>,
    pub ciuser: String,
    pub cipassword: String,
    #[serde(rename = "neosshkey_id")]
    pub keypair_id: u32,
    // pub keypair_name: String,
    pub sshkeys: String,
    pub osname: String,
    pub disk_size: String,
}

//
// Snapshots
//

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotAccountResource {
    #[serde(rename = "account_id")]
    pub id: u32,
    pub status: AccountStatus,
    pub extra_details: SnapshotExtraDetails,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotExtraDetails {
    pub name: String,
    pub description: String,
    pub region: String,
}
