#![forbid(unsafe_code)]
#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(trace_macros)]

mod backends;
mod environment;
mod error;
mod indicators;
mod loggers;
mod model;
mod strategies;
mod trader;
mod wallet;

use backends::Simulated;
use environment::Environment;
use error::Error;
use strategies::Custom;

//#[tokio::main(core_threads = 1, max_threads = 1)]
#[tokio::main]
async fn main() -> Result<(), Error> {
    Environment::<Simulated, Custom, _>::new().await.run().await;

    Ok(())
}
