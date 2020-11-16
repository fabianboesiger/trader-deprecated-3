#![forbid(unsafe_code)]
#![allow(incomplete_features)]
#![feature(const_generics)]

mod trader;
mod environment;
mod error;
mod indicators;
mod loggers;
mod managers;
mod model;
mod strategies;

use environment::Environment;
use error::Error;

//#[tokio::main(core_threads = 1, max_threads = 1)]
#[tokio::main]
async fn main() -> Result<(), Error> {
    Environment::new(strategies::Custom::new())
        .run()
        .await;

    Ok(())
}
