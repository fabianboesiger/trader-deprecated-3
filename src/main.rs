#![forbid(unsafe_code)]

mod environment;
mod error;
mod indicators;
mod loggers;
mod managers;
mod model;
mod traders;

use environment::Environment;
use error::Error;
use loggers::Web;
use managers::Simulated;
use model::Interval;
use traders::Custom;

#[tokio::main(core_threads = 1, max_threads = 1)]
async fn main() -> Result<(), Error> {
    Environment {
        trader: Custom::new(Interval::FiveMinutes),
        manager: Simulated::new(10.0, 0.001),
        logger: Web::new(([127, 0, 0, 1], 8000)),
    }
    .trade()
    .await;

    Ok(())
}
