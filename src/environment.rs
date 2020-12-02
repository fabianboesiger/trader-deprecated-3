use crate::{
    backends::Backend,
    indicators::Indicator,
    loggers::Logger,
    model::{Asset, Interval, Market, MAIN_ASSET},
    strategies::Strategy,
    trader::Trader,
    wallet::Wallet,
};
use openlimits::binance::Binance;
use tokio::sync::mpsc::{channel, unbounded_channel};
use tokio::task;

pub struct Environment<B, S, I>
where
    S: Strategy<I>,
    B: Backend,
    I: Indicator,
{
    backend: std::marker::PhantomData<B>,
    strategy: std::marker::PhantomData<S>,
    phantom: std::marker::PhantomData<I>,
    logger: crate::loggers::Web<([u8; 4], u16)>,
}

impl<B, S, I> Environment<B, S, I>
where
    S: Strategy<I>,
    B: Backend,
    I: Indicator,
{
    pub async fn new() -> Self {
        Environment {
            strategy: std::marker::PhantomData,
            backend: std::marker::PhantomData,
            phantom: std::marker::PhantomData,
            logger: crate::loggers::Web::new(([127, 0, 0, 1], 8000)),
        }
    }

    pub async fn run(self) {
        let exchange: &'static Binance = Box::leak(Box::new(Binance::new(false).await));

        let (order_sender, order_reciever) = channel(16);
        let (log_sender, log_receiver) = unbounded_channel();

        let tradable = Asset::all()
            .into_iter()
            .filter(|asset| *asset != MAIN_ASSET)
            .collect::<Vec<Asset>>();

        for asset in tradable {
            let trader = Trader::new(S::new(), Interval::FivteenMinutes);
            let sender = order_sender.clone();
            let market = Market {
                base: asset,
                quote: MAIN_ASSET,
            };

            task::spawn(async move {
                trader.run(exchange, market, sender).await;
            });
        }

        tokio::join! {
            Wallet::<B>::new(log_sender).await.run(exchange, order_reciever),
            self.logger.run(log_receiver)
        };
    }
}
