mod web;

pub use web::Web;

use async_trait::async_trait;
use tokio::sync::mpsc::UnboundedReceiver;

#[async_trait]
pub trait Logger: Send + Sync {
    async fn run(self, receiver: UnboundedReceiver<String>);
}
