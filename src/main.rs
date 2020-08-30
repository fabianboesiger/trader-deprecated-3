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
use traders::Bollinger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Environment {
        trader: Bollinger::new(Interval::FivteenMinutes, 20, 2.0),
        manager: Simulated::new(10.0, 0.001),
        logger: Web::new(([127, 0, 0, 1], 8000)),
    }
    .trade()
    .await;

    Ok(())
}
