mod log;
mod web;

pub use log::Log;
pub use web::Web;

use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;

#[async_trait]
pub trait Logger: Send + Sync {
    async fn run(self, receiver: Receiver<Log>);
}
