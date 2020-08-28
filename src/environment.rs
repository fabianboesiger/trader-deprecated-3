use crate::model::{Asset, Market, MAIN_ASSET};
use crate::{loggers::Logger, managers::Manager, traders::Trader};
use openlimits::binance::Binance;
use tokio::sync::mpsc::channel;
use tokio::task;

pub struct Environment<T: Trader, M: Manager, L: Logger> {
    pub trader: T,
    pub manager: M,
    pub logger: L,
}

impl<T: Trader, M: Manager, L: Logger> Environment<T, M, L> {
    pub async fn trade(&mut self) {
        let exchange: &'static Binance = Box::leak(Box::new(Binance::new(false)));

        let (sender, reciever) = channel(16);

        for asset in Asset::all()
            .into_iter()
            .filter(|asset| *asset != MAIN_ASSET)
        {
            let mut trader = self.trader.clone();
            let sender = sender.clone();
            let market = Market {
                base: asset,
                quote: MAIN_ASSET,
            };

            task::spawn(async move {
                trader.run(exchange, market, sender).await;
            });
        }

        self.manager.run(exchange, reciever).await;
    }
}
