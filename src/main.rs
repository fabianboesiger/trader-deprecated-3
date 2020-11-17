#![forbid(unsafe_code)]
#![allow(incomplete_features)]
#![feature(const_generics)]

mod environment;
mod error;
mod indicators;
mod loggers;
mod managers;
mod model;
mod strategies;
mod trader;

use environment::Environment;
use error::Error;

//#[tokio::main(core_threads = 1, max_threads = 1)]
#[tokio::main]
async fn main() -> Result<(), Error> {
    Environment::new(strategies::Custom::new()).await.run().await;

    Ok(())
}
