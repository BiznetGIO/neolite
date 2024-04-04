#![allow(dead_code)]
use std::env;

use anyhow::Context;
use neolite::{client::Client, config::Config, lite::Lite, snapshot::Snapshot};

async fn list(snapshot: Snapshot) -> anyhow::Result<()> {
    let snapshots = snapshot.list().await?;
    for snapshot in snapshots {
        println!("id: {}, name: {}", snapshot.id, snapshot.name);
    }

    Ok(())
}

async fn get(snapshot: Snapshot, id: u32) -> anyhow::Result<()> {
    let snapshot = snapshot.get(id).await?;
    println!("{}: {}", snapshot.id, snapshot.name);

    Ok(())
}

async fn delete(snapshot: Snapshot, id: u32) -> anyhow::Result<()> {
    snapshot.delete(id).await?;
    println!("::: Snapshot deleted.");

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
    let snapshot = Lite::new(client).snapshot().await?;

    // list(snapshot).await?;
    get(snapshot, id).await?;
    // list_with_status(vm).await?;
    // delete(vm, id).await?;

    Ok(())
}
