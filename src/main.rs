mod environment;
mod indicators;
mod loggers;
mod managers;
mod model;
mod traders;

use environment::Environment;
use loggers::Websocket;
use managers::Simulated;
use model::Interval;
use traders::Bollinger;

#[tokio::main]
async fn main() {
    Environment {
        trader: Bollinger::new(Interval::FivteenMinutes, 20, 2.0),
        manager: Simulated::new(10.0, 0.001),
        logger: Websocket::new(),
    }
    .trade()
    .await;
}
