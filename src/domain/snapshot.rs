use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json as json;

use super::account::{Account, AccountStatus};
use crate::{
    client::Client,
    keypair::KeypairResource,
    lite::BillingResource,
    plan::{Billing, PlanResource},
};

pub struct RestoreVirtualMachineOptions {
    pub plan: PlanResource,
    pub keypair: KeypairResource,
    pub billing: Billing,
    pub use_credit_card: bool,
    pub promocode: Option<String>,
}

pub struct SnapshotOpts {
    pub billing: Billing,
    pub use_credit_card: bool,
    pub promocode: Option<String>,
}

pub struct Snapshot {
    client: Arc<Client>,
}

impl Snapshot {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }
    pub async fn list(&self) -> Result<Vec<SnapshotResource>, crate::Error> {
        let mut snapshots: Vec<SnapshotResource> = Vec::new();

        let account = Account::new(Arc::clone(&self.client));
        let accounts = account.list_snapshot().await?;

        for account in accounts {
            // Skips terminated vm. It can't be accessed
            if account.status == AccountStatus::Terminated {
                continue;
            }
            let snapshot = SnapshotResource {
                id: account.id,
                name: account.extra_details.name,
            };
            snapshots.push(snapshot);
        }
        Ok(snapshots)
    }
    pub async fn create(
        &self,
        vm_id: u32,
        name: String,
        description: Option<String>,
        opts: &SnapshotOpts,
    ) -> Result<BillingResource, crate::Error> {
        let use_cc = match opts.use_credit_card {
            true => "yes",
            false => "no",
        };
        let body = json::json!({
            "name": &name,
            "description": &description,
            "cycle": opts.billing.cycle,
            "pay_invoice_with_cc": use_cc,
            "promocode": opts.promocode.as_deref(),
        });
        let response = self
            .client
            .post(&format!("/accounts/{vm_id}/snapshot"), body)
            .await?;
        let response: BillingResource = json::from_value(response)?;
        Ok(response)
    }
    pub async fn get(&self, id: u32) -> Result<SnapshotResource, crate::Error> {
        let account = Account::new(Arc::clone(&self.client));
        let account = account.get_snapshot(id).await?;
        let snapshot = SnapshotResource {
            id: account.id,
            name: account.extra_details.name,
        };
        Ok(snapshot)
    }
    pub async fn delete(&self, id: u32) -> Result<(), crate::Error> {
        self.client.delete(&format!("/snapshots/{}", id)).await?;
        Ok(())
    }
    pub async fn restore(&self, id: u32) -> Result<(), crate::Error> {
        self.client
            .put(&format!("/snapshots/accounts/{id}/restore"))
            .await?;
        Ok(())
    }
    pub async fn restore_with(
        &self,
        snapshot_id: u32,
        name: String,
        description: Option<String>,
        username: String,
        password: String,
        opts: &RestoreVirtualMachineOptions,
    ) -> Result<BillingResource, crate::Error> {
        let use_cc = match opts.use_credit_card {
            true => "yes",
            false => "no",
        };
        let body = json::json!({
            "ssh_and_console_user": &username,
            "console_password": &password,
            "vm_name": &name,
            "description": &description,
            "product_id": opts.plan.id,
            "keypair_id": opts.keypair.id,
            "cycle": opts.billing.cycle,
            "pay_invoice_with_cc": use_cc,
            "promocode": opts.promocode,
        });
        let response = self
            .client
            .post(&format!("/snapshots/accounts/{snapshot_id}/create"), body)
            .await?;
        let response: BillingResource = json::from_value(response)?;
        Ok(response)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotResource {
    pub id: u32,
    pub name: String,
}
