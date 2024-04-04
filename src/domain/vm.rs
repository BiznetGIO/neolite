use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json as json;

use super::account::{Account, AccountStatus};
use crate::{
    client::Client,
    keypair::KeypairResource,
    lite::BillingResource,
    os::OsResource,
    plan::{Billing, PlanResource},
};

pub struct VirtualMachineOptions {
    pub plan: PlanResource,
    pub keypair: KeypairResource,
    pub os: OsResource,
    pub billing: Billing,
    pub use_credit_card: bool,
    pub promocode: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
/// Terminated is not on the list because termindated VM can't be accessed.
pub enum VirtualMachineStatus {
    Active,
    Pending,
    Suspended,
}

impl VirtualMachineStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "Active",
            Self::Pending => "Pending",
            Self::Suspended => "Suspended",
        }
    }
}

pub struct VirtualMachine {
    client: Arc<Client>,
}

impl VirtualMachine {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }
    pub async fn list(&self) -> Result<Vec<VirtualMachineResource>, crate::Error> {
        let mut vms: Vec<VirtualMachineResource> = Vec::new();

        let account = Account::new(Arc::clone(&self.client));
        let accounts = account.list().await?;

        for account in accounts {
            // Skips terminated vm. It can't be accessed
            if account.status == AccountStatus::Terminated {
                continue;
            }
            let vm = self.get(account.id).await?;
            vms.push(vm);
        }
        Ok(vms)
    }
    pub async fn list_with_status(
        &self,
        status: VirtualMachineStatus,
    ) -> Result<Vec<VirtualMachineResource>, crate::Error> {
        let mut vms: Vec<VirtualMachineResource> = Vec::new();

        let status = match status {
            VirtualMachineStatus::Active => AccountStatus::Active,
            VirtualMachineStatus::Pending => AccountStatus::Pending,
            VirtualMachineStatus::Suspended => AccountStatus::Suspended,
        };
        let account = Account::new(Arc::clone(&self.client));
        let accounts = account.list_with_status(status).await?;

        for account in accounts {
            // Skips terminated vm. It can't be accessed
            if account.status == AccountStatus::Terminated {
                continue;
            }
            let vm = self.get(account.id).await?;
            vms.push(vm);
        }
        Ok(vms)
    }
    pub async fn get(&self, id: u32) -> Result<VirtualMachineResource, crate::Error> {
        let response = self
            .client
            .get(&format!("/accounts/{}/vm-details", id))
            .await?;
        let response: VirtualMachineResource = json::from_value(response)?;
        let response = response.with_client(Arc::clone(&self.client));
        // Set `id` to use `Account ID` instead of real `VM ID`.
        // The `Account ID` is used throughout the upstream REST API.
        let response = response.change_id(id);
        Ok(response)
    }
    pub async fn create(
        &self,
        name: String,
        description: Option<String>,
        username: String,
        password: String,
        opts: &VirtualMachineOptions,
    ) -> Result<BillingResource, crate::Error> {
        let use_cc = match opts.use_credit_card {
            true => "yes",
            false => "no",
        };
        let body = json::json!({
            "ssh_and_console_user":  &username,
            "console_password":  &password,
            "vm_name": &name,
            "description": &description,
            "product_id": opts.plan.id,
            "select_os": &opts.os.name,
            "keypair_id": opts.keypair.id,
            "cycle": opts.billing.cycle,
            "pay_invoice_with_cc": use_cc,
            "promocode": opts.promocode,
        });
        let response = self.client.post("", body).await?;
        let response: BillingResource = json::from_value(response)?;
        Ok(response)
    }
    pub async fn delete(&self, id: u32) -> Result<(), crate::Error> {
        self.client.delete(&format!("/{}", id)).await?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualMachineResource {
    #[serde(rename = "vmid")]
    pub id: u32,
    pub name: String,
    pub status: String,
    pub uptime: i32,
    pub maxdisk: i64,
    pub maxmem: i64,
    pub mem: i32,
    pub cpus: i32,

    #[serde(skip)]
    client: Arc<Client>,
}

impl VirtualMachineResource {
    fn with_client(mut self, client: Arc<Client>) -> Self {
        self.client = client;
        self
    }
    fn change_id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }
    pub async fn change_keypair(&self, keypair_id: u32) -> Result<(), crate::Error> {
        let body = json::json!({ "keypair_id": keypair_id });
        self.client
            .put_with_body(&format!("/accounts/{}/keypair", self.id), body)
            .await?;
        Ok(())
    }
    pub async fn change_name(&self, name: &str) -> Result<(), crate::Error> {
        let body = json::json!({ "name": name });
        self.client
            .put_with_body(&format!("/accounts/{}/change-vm-name", self.id), body)
            .await?;
        Ok(())
    }
    pub async fn change_plan(&self, plan_id: u32) -> Result<BillingResource, crate::Error> {
        let body = json::json!({ "new_product_id": plan_id });
        let response = self
            .client
            .post(&format!("/accounts/{}/change-package", self.id), body)
            .await?;
        let response: BillingResource = json::from_value(response)?;
        Ok(response)
    }
    pub async fn change_storage(&self, size: u32) -> Result<BillingResource, crate::Error> {
        let body = json::json!({ "disk_size": size });
        let response = self
            .client
            .put_with_body(&format!("/accounts/{}/storage", self.id), body)
            .await?;
        let response: BillingResource = json::from_value(response)?;
        Ok(response)
    }
    pub async fn start(&self) -> Result<(), crate::Error> {
        self.change_state("start").await?;
        Ok(())
    }
    pub async fn suspend(&self) -> Result<(), crate::Error> {
        self.change_state("suspend").await?;
        Ok(())
    }
    pub async fn resume(&self) -> Result<(), crate::Error> {
        self.change_state("resume").await?;
        Ok(())
    }
    pub async fn reset(&self) -> Result<(), crate::Error> {
        self.change_state("reset").await?;
        Ok(())
    }
    pub async fn shutdown(&self) -> Result<(), crate::Error> {
        self.change_state("shutdown").await?;
        Ok(())
    }
    pub async fn stop(&self) -> Result<(), crate::Error> {
        self.change_state("stop").await?;
        Ok(())
    }
    pub async fn rebuild(&self, os: &OsResource) -> Result<(), crate::Error> {
        let body = json::json!({ "name": os.name });
        self.client
            .put_with_body(&format!("/accounts/{}/rebuild", self.id), body)
            .await?;
        Ok(())
    }
    async fn change_state(&self, state: &str) -> Result<(), crate::Error> {
        self.client
            .put(&format!("/accounts/{}/vm-state/{}", self.id, state))
            .await?;
        Ok(())
    }
}
