use crate::model::{Asset, Market, MAIN_ASSET};
use crate::{loggers::Logger, managers::Manager, traders::Trader};
use openlimits::binance::Binance;
use tokio::sync::mpsc::unbounded_channel;
use tokio::task;

pub struct Environment<T: Trader, M: Manager, L: Logger> {
    pub trader: T,
    pub manager: M,
    pub logger: L,
}

impl<T: Trader, M: Manager, L: Logger> Environment<T, M, L> {
    pub async fn trade(self) {
        let exchange: &'static Binance = Box::leak(Box::new(Binance::new(false)));

        let (order_sender, order_reciever) = unbounded_channel();
        let (message_sender, message_reciever) = unbounded_channel();

        for asset in Asset::all()
            .into_iter()
            .filter(|asset| *asset != MAIN_ASSET)
        {
            let trader = self.trader.clone();
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
            self.manager.run(exchange, order_reciever, message_sender),
            self.logger.run(message_reciever)
        };
    }
}
