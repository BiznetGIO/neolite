# Guide

## Common workflow

- Create NEO Lite Virtual Machine.
  - Check available products `product.list()`.
  - Select preferred product `product.get(1538)`.
  - Select preferred billing cycle `product_resource.get_billing("Monthly")`.
  - Check IP availability `ip.is_available()`.
  - Select preferred OS `os.get(1001)`.
  - Create or Select existing keypair `keypair.create("gandalf0")`.
  - Create a virtual Machine `lite.create()`.
