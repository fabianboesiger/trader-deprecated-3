mod environment;

use environment::Environment;

#[tokio::main]
async fn main() {
    Environment::new().run().await;
}
