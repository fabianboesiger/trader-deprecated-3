use super::{Candlestick, Order, Asset, indicators::BollingerBands};
use num_traits::cast::ToPrimitive;
use futures::{stream, StreamExt};
use openlimits::binance::{
    client::websocket::BinanceWebsocket,
    model::{
        websocket::{BinanceWebsocketMessage, Subscription},
        KlineParams, KlineSummaries, Paginator
    },
    Binance,
};
use tokio::sync::mpsc::Sender;

enum Position {
    Long,
    Short,
}

pub struct Trader<'a, const BASE: Asset, const QUOTE: Asset> {
    exchange: &'a Binance,
    sender: Sender<Order>,
    position: Position,
    bollinger_bands: BollingerBands,
}

impl<'a, const BASE: Asset, const QUOTE: Asset> Trader<'a, BASE, QUOTE> {
    pub fn new(exchange: &'a Binance, sender: Sender<Order>) -> Self {
        Trader {
            exchange,
            sender,
            position: Position::Short,
            bollinger_bands: BollingerBands::new(20, 2.0),
        }
    }

    pub async fn run(&mut self) {
        let interval = String::from("15m");
 
        // Get historical data using REST API.
        let params = KlineParams {
            symbol: format!("{}{}", BASE, QUOTE),
            interval: interval.clone(),
            paginator: Some(Paginator {
                start_time: None,
                end_time: None,
                limit: Some(1000),
                from_id: None,
                order_id: None,
            }),
        };
        let response = self.exchange.get_klines(&params).await.unwrap();
        let KlineSummaries::AllKlineSummaries(summaries) = response;
        let historical_candlesticks = stream::iter(
            summaries
                .into_iter()
                .map(Candlestick::from)
                .collect::<Vec<Candlestick<BASE, QUOTE>>>(),
        );

        // Get live data using websocket API.
        let mut websocket = BinanceWebsocket::new();
        let sub = Subscription::Candlestick(format!("{}{}", BASE, QUOTE).to_lowercase(), interval);
        websocket.subscribe(sub).await.unwrap();
        let live_candlesticks = websocket.filter_map(|message| async move {
            if let BinanceWebsocketMessage::Candlestick(candlestick) = message.unwrap() {
                if candlestick.kline.is_final_bar {
                    return Some(Candlestick::from(candlestick.kline));
                }
            }
            None
        });

        // Chain and handle the candlesticks.
        let mut stream = historical_candlesticks.chain(live_candlesticks).boxed();
        while let Some(candlestick) = stream.next().await {
            if let Some((upper, lower)) = self.bollinger_bands.update(&candlestick) {
                match self.position {
                    Position::Long => if candlestick.high.to_f64().unwrap() > upper {
                        self.position = Position::Short;
                        self.sender.send(Order::Sell(self.symbol.clone(), candlestick.close)).await.unwrap();
                    },
                    Position::Short => if candlestick.low.to_f64().unwrap() < lower {
                        self.position = Position::Long;
                        self.sender.send(Order::Buy(self.symbol.clone(), candlestick.close)).await.unwrap();
                    }
                }
            }
            self.sender.send(Order::Update(self.symbol.clone(), candlestick.close)).await.unwrap();
        }
        
    }
}
