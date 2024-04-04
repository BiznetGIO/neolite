#![allow(dead_code)]
use std::env;

use anyhow::Context;
use neolite::{
    client::Client, config::Config, lite::Lite, vm::VirtualMachine, vm::VirtualMachineStatus,
};

async fn list(vm: VirtualMachine) -> anyhow::Result<()> {
    let vms = vm.list().await?;
    for vm in vms {
        println!("id: {}, name: {}, status: {}", vm.id, vm.name, vm.status,);
    }

    Ok(())
}

async fn list_with_status(vm: VirtualMachine) -> anyhow::Result<()> {
    let vms = vm.list_with_status(VirtualMachineStatus::Active).await?;
    for vm in vms {
        println!("id: {}, name: {}, status: {}", vm.id, vm.name, vm.status,);
    }

    Ok(())
}

async fn get(vm: VirtualMachine, id: u32) -> anyhow::Result<()> {
    let vm = vm.get(id).await?;
    println!("{}: {}", vm.id, vm.name);

    Ok(())
}

async fn delete(vm: VirtualMachine, id: u32) -> anyhow::Result<()> {
    vm.delete(id).await?;
    println!("::: Virtual machine deleted.");

    Ok(())
}

async fn stop(vm: VirtualMachine, id: u32) -> anyhow::Result<()> {
    let vm = vm.get(id).await?;
    vm.stop().await?;
    println!("::: Virtual machine stopped.");

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
    let vm = Lite::new(client).vm().await?;

    get(vm, id).await?;
    // list(vm).await?;
    // list_with_status(vm).await?;
    // delete(vm, id).await?;

    Ok(())
}
