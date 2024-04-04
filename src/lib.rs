// #![deny(clippy::unwrap_used)]

pub mod client;
pub mod config;
pub mod domain;
pub mod error;

pub use error::Error;

pub use domain::{
    keypair, lite,
    products::{ip, os, plan},
    snapshot, vm,
};
