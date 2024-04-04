use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{client::Client, keypair::Keypair, plan::Plan, snapshot::Snapshot, vm::VirtualMachine};

pub struct Lite {
    client: Arc<Client>,
}

impl Lite {
    pub fn new(client: Client) -> Self {
        Self {
            client: Arc::new(client),
        }
    }
    pub async fn keypair(&self) -> Result<Keypair, crate::Error> {
        Ok(Keypair::new(Arc::clone(&self.client)))
    }
    pub async fn plan(&self) -> Result<Plan, crate::Error> {
        Ok(Plan::new(Arc::clone(&self.client)))
    }
    pub async fn vm(&self) -> Result<VirtualMachine, crate::Error> {
        Ok(VirtualMachine::new(Arc::clone(&self.client)))
    }
    pub async fn snapshot(&self) -> Result<Snapshot, crate::Error> {
        Ok(Snapshot::new(Arc::clone(&self.client)))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BillingResource {
    pub order_id: String,
    pub account_id: String,
}
