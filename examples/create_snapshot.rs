#![allow(dead_code)]
use std::env;

use anyhow::Context;
use neolite::{client::Client, config::Config, lite::Lite, snapshot::SnapshotOpts};

async fn create(client: Client, vm_id: u32) -> anyhow::Result<()> {
    let lite = Lite::new(client);

    // (1) Select preferred billing cycle
    let product = lite.plan().await?;
    let product_resource = product.get_vm(1538).await?;
    let billing_resource = product_resource.get_billing("Monthly").await?;
    println!(
        "::: Billing. label: {}, price: {}",
        billing_resource.label, billing_resource.price,
    );

    // (2) Create a virtual machine snapshot
    let snapshot = lite.snapshot().await?;
    let opts = SnapshotOpts {
        billing: billing_resource,
        use_credit_card: false,
        promocode: None,
    };
    let billing_resource = snapshot
        .create(
            vm_id,
            "snapshot-from-sdk".to_string(),
            Some("Snapshot from SDK".to_string()),
            &opts,
        )
        .await?;
    println!(
        "::: Snapshot. account id: {}, order id: {}",
        billing_resource.account_id, billing_resource.order_id
    );

    Ok(())
}

async fn restore_with(client: Client) -> anyhow::Result<()> {
    let lite = Lite::new(client);

    // (1) Select preferred plan
    // let resource = plan.list().await?;
    let plan = lite.plan().await?;
    let plan_resource = plan.get_vm(1538).await?;
    println!(
        "::: plan. id: {}, name: {}",
        plan_resource.id, plan_resource.name,
    );

    // (2) Select preferred billing cycle
    let billing_resource = plan_resource.get_billing("Monthly").await?;
    println!(
        "::: Billing. label: {}, price: {}",
        billing_resource.label, billing_resource.price,
    );

    // (3) Create or Select existing keypair
    let keypair = lite.keypair().await?;
    let keypair_resource = keypair.create("gandalf0").await?;
    println!(
        "::: Keypair. id: {}, name: {}",
        keypair_resource.id, keypair_resource.name,
    );

    // (4) Create a virtual machine snapshot
    let snapshot = lite.snapshot().await?;
    let opts = neolite::snapshot::RestoreVirtualMachineOptions {
        plan: plan_resource,
        keypair: keypair_resource,
        billing: billing_resource,
        use_credit_card: false,
        promocode: None,
    };
    let snapshot_id = 123;
    let billing_resource = snapshot
        .restore_with(
            snapshot_id,
            "thorin-os2".to_string(),
            Some("Thorin Virtual Machine".to_string()),
            "thethorin".to_string(),
            "SpeakFriendAndEnter123".to_string(),
            &opts,
        )
        .await?;
    println!(
        "::: VM created. account id: {}, order id: {}",
        billing_resource.account_id, billing_resource.order_id
    );

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    dotenvy::from_filename("./examples/.env")?;

    let url = "https://api.portal.biznetgio.dev/v1/neolites".parse::<http::Uri>()?;
    let token = env::var("TOKEN").context("TOKEN env not found.")?;
    let id = env::var("VM_ID").context("VM_ID env not found.")?;
    let id: u32 = id.parse()?;

    let config = Config::new(url, &token);
    let client = Client::new(config)?;

    create(client, id).await?;
    // restore_with(client).await?;

    Ok(())
}
