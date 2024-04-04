#![allow(dead_code)]
use std::env;

use anyhow::Context;
use neolite::{client::Client, config::Config, keypair::Keypair, lite::Lite};

async fn list(keypair: Keypair) -> anyhow::Result<()> {
    let keys = keypair.list().await?;
    for key in keys {
        println!("{}: {}", key.id, key.name);
    }
    Ok(())
}

async fn get(keypair: Keypair, id: u32) -> anyhow::Result<()> {
    let key = keypair.get(id).await?;
    println!("id: {}, name: {}", key.id, key.name);
    Ok(())
}

async fn create(keypair: Keypair) -> anyhow::Result<()> {
    let key = keypair.create("gandalf0").await?;
    println!("{:?}", key);
    println!("{}", key.name);
    Ok(())
}

async fn delete(keypair: Keypair, id: u32) -> anyhow::Result<()> {
    keypair.delete(id).await?;
    println!("::: Keypair deleted.");

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    dotenvy::from_filename("./examples/.env")?;

    let url = "https://api.portal.biznetgio.dev/v1/neolites".parse::<http::Uri>()?;
    let token = env::var("TOKEN").context("TOKEN env not found.")?;
    let id = env::var("KEYPAIR_ID").context("KEYPAIR_ID env not found.")?;
    let id: u32 = id.parse()?;

    let config = Config::new(url, &token);
    let client = Client::new(config)?;
    let keypair = Lite::new(client).keypair().await?;

    // list(keypair).await?;
    // create(keypair).await?;
    get(keypair, id).await?;
    // delete(keypair, id).await?;

    Ok(())
}
