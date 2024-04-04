use std::env;

use anyhow::Context;
use neolite::{client::Client, config::Config, lite::Lite, vm::VirtualMachineOptions};

async fn create(client: Client) -> anyhow::Result<()> {
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

    // (3) Check IP availability
    let ip = plan_resource.ip().await?;
    let is_ip_available = ip.is_available().await?;
    if !is_ip_available {
        return Err(anyhow::anyhow!("IP is not available"));
    }
    println!("::: IP availability: {}", is_ip_available);

    // (4) Select preferred OS
    let os = plan_resource.os().await?;
    // let oses = os.list().await?;
    let os_resource = os.get(1001).await?;
    println!("::: OS. id: {}, name: {}", os_resource.id, os_resource.name);

    // (5) Create or Select existing keypair
    let keypair = lite.keypair().await?;
    let keypair_resource = keypair.create("gandalf0").await?;
    println!(
        "::: Keypair. id: {}, name: {}",
        keypair_resource.id, keypair_resource.name,
    );

    // (6) Create a virtual Machine
    let vm = lite.vm().await?;
    let opts = VirtualMachineOptions {
        plan: plan_resource,
        os: os_resource,
        keypair: keypair_resource,
        billing: billing_resource,
        use_credit_card: false,
        promocode: None,
    };
    let billing_resource = vm
        .create(
            "thorin-os2".to_string(),
            Some("Thorin Virtual Machine".to_string()),
            "thethorin".to_string(),
            "SpeakFriendAndEnter123".to_string(),
            &opts,
        )
        .await?;
    println!(
        "::: NeoLite VM. account id: {}, order id: {}",
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
    let config = Config::new(url, &token);

    let client = Client::new(config)?;
    create(client).await?;

    Ok(())
}
