#![forbid(unsafe_code)]
#![allow(incomplete_features)]
#![feature(const_generics)]

mod environment;
mod error;
mod indicators;
mod loggers;
mod backends;
mod model;
mod strategies;
mod trader;
mod wallet;

use environment::Environment;
use error::Error;
use backends::Simulated;
use strategies::Custom;

//#[tokio::main(core_threads = 1, max_threads = 1)]
#[tokio::main]
async fn main() -> Result<(), Error> {
    Environment::<Simulated, Custom, _>::new().await.run().await;

    Ok(())
}
