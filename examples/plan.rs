#![allow(dead_code)]
use std::env;

use anyhow::Context;
use neolite::{client::Client, config::Config, lite::Lite, plan::Plan};

async fn list(plan: Plan) -> anyhow::Result<()> {
    let plans = plan.list_vm().await?;
    for p in plans {
        println!("{}: {}", p.name, p.id);
    }
    Ok(())
}

async fn get(plan: Plan, id: u32) -> anyhow::Result<()> {
    let plan = plan.get_vm(id).await?;
    println!("{}: {}", plan.name, plan.id);
    Ok(())
}

/// List available OS for the VM
async fn list_os(plan: Plan, id: u32) -> anyhow::Result<()> {
    let plan = plan.get_vm(id).await?;
    let os = plan.os().await?;
    let all_oses = os.list().await?;
    println!("{:?}", all_oses);
    Ok(())
}

async fn get_os(plan: Plan, id: u32) -> anyhow::Result<()> {
    let plan = plan.get_vm(id).await?;
    let os = plan.os().await?;
    let os = os.get(1001).await?;
    println!("{}", os.name);
    Ok(())
}

async fn check_ip_availability(plan: Plan, id: u32) -> anyhow::Result<()> {
    let plan = plan.get_vm(id).await?;
    let ip = plan.ip().await?;
    println!(
        "Ip public availability status: {:?}",
        ip.is_available().await?
    );
    Ok(())
}

//
// Snapshots
//

async fn list_snapshot(plan: Plan) -> anyhow::Result<()> {
    let plans = plan.list_snapshot().await?;
    for p in plans {
        println!("{}: {}", p.name, p.id);
    }
    Ok(())
}

async fn get_snapshot(plan: Plan, id: u32) -> anyhow::Result<()> {
    let p = plan.get_snapshot(id).await?;
    println!("{}: {}", p.name, p.id);
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    dotenvy::from_filename("./examples/.env")?;

    let url = "https://api.portal.biznetgio.dev/v1/neolites".parse::<http::Uri>()?;
    let token = env::var("TOKEN").context("TOKEN env not found.")?;
    let config = Config::new(url, &token);
    let client = Client::new(config)?;
    let id = env::var("PLAN_ID").context("PLAN_ID env not found.")?;
    let id: u32 = id.parse()?;

    let plan = Lite::new(client).plan().await?;

    // list(plan).await?;
    // get(plan, id).await?;
    // list_os(plan, id).await?;
    // get_os(plan, id).await?;
    // check_ip_availability(plan, id).await?;

    // list_snapshot(plan).await?;
    get_snapshot(plan, id).await?;

    Ok(())
}
