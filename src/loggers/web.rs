use super::{Log, Logger};
use async_trait::async_trait;
use futures::{FutureExt, SinkExt, StreamExt};
use sqlx::PgPool;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::mpsc;
use tokio::sync::{mpsc::UnboundedReceiver, Mutex};
use warp::ws::{Message, WebSocket};
use warp::Filter;

type Senders = Arc<
    Mutex<
        Vec<futures::stream::SplitSink<warp::filters::ws::WebSocket, warp::filters::ws::Message>>,
    >,
>;

pub struct Web<A: Into<SocketAddr> + Send + Sync + 'static> {
    address: A,
}

impl<A: Into<SocketAddr> + Send + Sync + 'static> Web<A> {
    pub fn new(address: A) -> Self {
        Self { address }
    }
}

#[async_trait]
impl<A: Into<SocketAddr> + Send + Sync + 'static> Logger for Web<A> {
    async fn run(self, mut receiver: UnboundedReceiver<Log>) {
        let address = self.address;

        dotenv::dotenv().ok();

        let pool = Arc::new(
            PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
                .await
                .unwrap(),
        );

        let senders = Arc::new(Mutex::new(Vec::new()));
        let senders_clone = senders.clone();

        let socket = warp::path("socket")
            .and(warp::path::end())
            .and(warp::ws())
            .and(warp::any().map(move || senders_clone.clone()))
            .and(warp::any().map(move || pool.clone()))
            .map(|ws: warp::ws::Ws, senders: Senders, pool: Arc<PgPool>| {
                ws.on_upgrade(move |ws| connect(ws, senders, pool))
            });

        let routes = warp::fs::dir("web").or(socket);

        tokio::join! {
            async {
                while let Some(log) = receiver.recv().await {
                    let data = serde_json::to_string(&log).unwrap();

                    let mut senders = senders.lock().await;

                    let mut i: usize = 0;
                    while i != senders.len() {
                        if let Err(_) = senders[i].send(Message::text(&data)).await {
                            let _removed = senders.remove(i);
                        } else {
                            i += 1;
                        }
                    }
                }
            },
            warp::serve(routes).run(address)
        };
    }
}

async fn connect(ws: WebSocket, senders: Senders, pool: Arc<PgPool>) {
    let (sender, _receiver) = ws.split();

    //sender.send(Message::text(&data));

    senders.lock().await.push(sender);
}