use crate::model::{Asset, Market, MAIN_ASSET, Interval};
use crate::{indicators::Indicator, loggers::Logger, managers::Manager, trader::Trader, strategies::Strategy};
use openlimits::binance::Binance;
use tokio::sync::{mpsc::channel, Barrier};
use tokio::task;
use std::sync::Arc;

pub struct Environment<S, I>
where
    S: Strategy<I>,
    I: Indicator
{
    strategy: S,
    phantom: std::marker::PhantomData<I>,
    manager: crate::managers::Simulated,
    logger: crate::loggers::Web<([u8; 4], u16)>,
}

impl<S, I> Environment<S, I>
where
    S: Strategy<I>,
    I: Indicator
{
    pub fn new(strategy: S) -> Self {
        Environment {
            strategy,
            phantom: std::marker::PhantomData,
            manager: crate::managers::Simulated::new(10.0, 0.001),
            logger: crate::loggers::Web::new(([127, 0, 0, 1], 8000)),
        }
    }

    pub async fn run(self) {
        println!("start trading");

        let exchange: &'static Binance = Box::leak(Box::new(Binance::new(false).await));

        let (order_sender, order_reciever) = channel(1);
        let (message_sender, message_reciever) = channel(16);

        let tradable = Asset::all()
            .into_iter()
            .filter(|asset| *asset != MAIN_ASSET)
            .collect::<Vec<Asset>>();

        let barrier = Arc::new(Barrier::new(tradable.len()));

        for asset in tradable
        {
            let trader = Trader::new(self.strategy.clone(), Interval::ThreeMinutes);
            let barrier = barrier.clone();
            let sender = order_sender.clone();
            let market = Market {
                base: asset,
                quote: MAIN_ASSET,
            };

            task::spawn(async move {
                trader.run(exchange, market, barrier, sender).await;
            });
        }

        tokio::join! {
            self.manager.run(exchange, order_reciever, message_sender),
            self.logger.run(message_reciever)
        };
    }
}
