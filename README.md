<div align="center">
<h1>neolite</h1>

NEO Lite SDK.

<a href="https://github.com/BiznetGIO/neolite/actions/workflows/ci.yml">
<img src="https://github.com/BiznetGIO/neolite/actions/workflows/ci.yml/badge.svg">
</a>
<a href="https://crates.io/crates/neolite">
<img src="https://img.shields.io/crates/v/neolite.svg">
</a>

</div>

---

The `neolite` SDK makes it easy to work with Biznet Gio's [NEO Lite](https://www.biznetgio.com/product/neo-lite) service. With NEO Lite SDK, developers can effortlessly manage and control their VPS instances for hosting websites and applications.

## Usage

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = "https://api.portal.biznetgio.dev/v1/neolites".parse::<http::Uri>()?;
    let token = env::var("TOKEN").context("TOKEN env not found.")?;

    let config = Config::new(url, &token);
    let client = Client::new(config)?;

    let keypair = Lite::new(client).keypair().await?;
    let key = keypair.create("gandalf0").await?;
    println!("{}", key.name);
    Ok(())
}
```

To learn more, see other [examples](/examples).

## Development

```bash
git clone https://github.com/BiznetGIO/neolite
cd neolite

# Run unit tests and integration tests
just comply
```

## Contributing

To learn more read the [contributing guide](docs/dev/README.md)
