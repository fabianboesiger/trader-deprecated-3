use super::{Log, Logger};
use async_trait::async_trait;
use futures::StreamExt;
use std::{
    net::SocketAddr,
    sync::Arc,
};
use tokio::sync::{
    mpsc::{channel, UnboundedReceiver, Receiver, Sender},
    Mutex, RwLock,
};
use warp::{
    ws::{Message, WebSocket},
    Filter,
};

type Senders = Arc<Mutex<Vec<Sender<Result<Message, warp::Error>>>>>;
type Cache = Arc<RwLock<String>>;

pub struct Web<A: Into<SocketAddr> + Send + Sync + 'static> {
    address: A,
    cache: Cache,
}

impl<A: Into<SocketAddr> + Send + Sync + 'static> Web<A> {
    pub fn new(address: A) -> Self {
        Self {
            address,
            cache: Arc::new(RwLock::new(String::new())),
        }
    }
}

#[async_trait]
impl<A: Into<SocketAddr> + Send + Sync + 'static> Logger for Web<A> {
    async fn run(self, mut receiver: UnboundedReceiver<String>) { 
        let address = self.address;      
        let senders = Arc::new(Mutex::new(Vec::new()));

        let senders_clone = senders.clone();
        let cache_clone = self.cache.clone();

        let socket = warp::path("socket")
            .and(warp::path::end())
            .and(warp::ws())
            .and(warp::any().map(move || senders_clone.clone()))
            .and(warp::any().map(move || cache_clone.clone()))
            .map(
                |ws: warp::ws::Ws, senders: Senders, cache: Cache| {
                    ws.on_upgrade(move |ws| connect(ws, senders, cache))
                },
            );

        let routes = warp::fs::dir("web").or(socket);

        let cache = self.cache;

        tokio::join! {
            async move {
                while let Some(log) = receiver.recv().await {
                    {
                        let mut senders = senders.lock().await;

                        let mut i: usize = 0;
                        while i != senders.len() {
                            if let Err(_) = senders[i].send(Ok(Message::text(&log))).await {
                                let _removed = senders.remove(i);
                            } else {
                                i += 1;
                            }
                        }
                    }
                    

                    *cache.write().await = log;
                }
            },
            warp::serve(routes).run(address)
        };
    }
}

async fn connect(ws: WebSocket, senders: Senders, cache: Cache) {
    println!("conn");

    let (ws_sender, _ws_receiver) = ws.split();
    let (mut sender, receiver) = channel(16);

    tokio::task::spawn(receiver.forward(ws_sender));
    /*
    let mut all_trades = Log::select_all_trades(&pool);

    while let Some(Ok(trade)) = all_trades.next().await {
        let data = serde_json::to_string(&trade).unwrap();
        if let Err(_) = sender.send(Ok(Message::text(&data))).await {
            continue;
        }
    }
    */
    /*
    let values = cache.read().await;
    for (_market, value) in &*values {
        let data = serde_json::to_string(&Log::Value(*value)).unwrap();
        if let Err(_) = sender.send(Ok(Message::text(&data))).await {
            continue;
        }
    }
    */

    sender.send(Ok(Message::text(&*cache.read().await))).await.ok();

    senders.lock().await.push(sender);
}
