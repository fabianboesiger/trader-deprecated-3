mod trader;
mod candlestick;
mod order;
mod indicators;

use trader::Trader;
use candlestick::Candlestick;
use order::Order;
use openlimits::binance::Binance;
use rust_decimal::Decimal;
use tokio::sync::mpsc::channel;
use tokio::task;
use std::{
    ops::{Add, Sub, Mul, Div, Deref},
    fmt,
    collections::HashMap,
    hash::Hash,
};

trait Asset {
    const NAME: &'static str;
}

macro_rules! gen_types {
    (
        $trait_name:ident {
            $($impl_name:ident),* $(,)?
        }
    ) => {
        $(
            struct $impl_name;
            impl $trait_name for $impl_name {
                const NAME: &'static str = stringify!($impl_name);
            }
        )*
    };
}

gen_types! {
    Asset {
        USDT,
        IOS,
        DOGE,
        VET,
        TRX,
    }
}

/*
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Asset {
    USDT,
    IOS,
    DOGE,
    VET,
    TRX,
}

impl Asset {
    pub fn from_string(string: &str) -> Option<Asset> {
        match string {
            "USDT" => Some(Asset::USDT),
            "IOS" => Some(Asset::IOS),
            "DOGE" => Some(Asset::DOGE),
            "VET" => Some(Asset::VET),
            "TRX" => Some(Asset::TRX),
            _ => None,
        }
    }
}


impl fmt::Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Asset::USDT => "USDT",
            Asset::IOS => "IOS",
            Asset::DOGE => "DOGE",
            Asset::VET => "VET",
            Asset::TRX => "TRX",
        })
    }
}
*/
#[derive(Debug)]
pub struct Quantity<A: Asset>(Decimal);

impl<A: Asset> fmt::Display for Quantity<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, A::NAME)
    }
}

impl<A: Asset> Add for Quantity<A> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl<A: Asset> Sub for Quantity<A> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl<B: Asset, Q: Asset> Mul<Value<B, Q>> for Quantity<B> {
    type Output = Quantity<Q>;

    fn mul(self, rhs: Value<B, Q>) -> Self::Output {
        Quantity::<Q>(self.0 * rhs.0)
    }
}

impl<B: Asset, Q: Asset> Div<Value<B, Q>> for Quantity<Q> {
    type Output = Quantity<B>;

    fn div(self, rhs: Value<B, Q>) -> Self::Output {
        Quantity::<B>(self.0 / rhs.0)
    }
}

#[derive(Debug)]
pub struct Value<B: Asset, Q: Asset>(Decimal);

impl<B: Asset, Q: Asset> fmt::Display for Value<B, Q> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}/{}", self.0, B::NAME, Q::NAME)
    }
}

pub struct Environment {
    exchange: &'static Binance,
    quantities: HashMap<AssetSymbol, Decimal>,
    values: HashMap<AssetSymbol, Decimal>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            exchange: Box::leak(Box::new(Binance::new(false))),
            quantities: HashMap::new(),
            values: HashMap::new(),
        }
    }

    pub async fn run(&mut self) {
        let main_asset = USDT;

        let exchange_info = self.exchange.get_exchange_info().await.unwrap();

        let (sender, mut reciever) = channel(16);

        for symbol in exchange_info.symbols
            .into_iter()
            .filter(|symbol| 
                ["IOSTUSDT", "DOGEUSDT",
                "VETUSDT", "TRXUSDT", "ADAUSDT",
                "ZILUSDT", "LENDUSDT", "XLMUSDT",
                "XRPUSDT", "THETAUSDT", "BATUSDT",
                "ALGOUSDT", "DOTUSDT", "ONTUSDT",
                "SXPUSDT", "ZRXUSDT", "LINKUSDT",
                "RLCUSDT", "XTZUSDT", "IOTAUSDT",
                "EOSUSDT", "OMGUSDT", "ATOMUSDT",
                "WAVESUSDT", "QTUMUSDT", "BANDUSDT",
                "KAVAUSDT", "KNCUSDT", "ETCUSDT"]
                    .contains(&symbol.symbol.as_str())
            )
        {
            let sender = sender.clone();
            let exchange = self.exchange;
            let base = Asset::from_string(symbol.base_asset.as_str());
            let quote = Asset::from_string(symbol.quote_asset.as_str());    
            
            if let (Some(B), Some(quote)) = (B, quote) {
                self.quantities.insert(B, Decimal::from(0));
                self.quantities.insert(quote, Decimal::from(0));

                Trader::new(exchange, sender).run().await;
    
                task::spawn(async move {
                    Trader::<base, quote>::new(exchange, sender, symbol).run().await;
                });
            }
        }

        self.quantities.insert(main_asset.clone(), Decimal::from(1000));
        self.values.insert(main_asset.clone(), Decimal::from(1));

        while let Some(order) = reciever.recv().await {
            match order {
                Order::Buy(market, value) => {
                    let buy_amount = if &market.B == &main_asset {
                            *self.quantities.get(&market.quote).unwrap() / value
                        } else
                        if &market.quote == &main_asset {
                            Decimal::from(100).min(*self.quantities.get(&market.quote).unwrap()) / value
                        } else {
                            unreachable!();
                        };

                    /*println!(
                        "BUY\t {} + {} {}\t {} - {} {}\t {}",
                        self.quantities.get(&market.B).unwrap(),
                        buy_amount,
                        market.B,
                        self.quantities.get(&market.quote).unwrap(),
                        buy_amount * value,
                        market.quote,
                        value
                    );*/
                    *self.quantities.get_mut(&market.B).unwrap() += buy_amount;
                    *self.quantities.get_mut(&market.quote).unwrap() -= buy_amount * value;

                    if &market.B == &main_asset {
                        self.values.insert(market.quote.clone(), Decimal::from(1) / value);
                    } else
                    if &market.quote == &main_asset {
                        self.values.insert(market.B.clone(), value);
                    } else {
                        unreachable!();
                    }
                },
                Order::Sell(market, value) => {
                    let sell_amount = if &market.B == &main_asset {
                            Decimal::from(100).min(*self.quantities.get(&market.B).unwrap())
                        } else
                        if &market.quote == &main_asset {
                            *self.quantities.get(&market.B).unwrap()
                        } else {
                            unreachable!();
                        };

                    /*println!(
                        "SELL\t {} - {} {}\t {} + {} {}\t {}",
                        self.quantities.get(&market.B).unwrap(),
                        sell_amount,
                        market.B,
                        self.quantities.get(&market.quote).unwrap(),
                        sell_amount * value,
                        market.quote,
                        value
                    );*/
                    *self.quantities.get_mut(&market.B).unwrap() -= sell_amount;
                    *self.quantities.get_mut(&market.quote).unwrap() += sell_amount * value;

                    if &market.B == &main_asset {
                        self.values.insert(market.quote.clone(), Decimal::from(1) / value);
                    } else
                    if &market.quote == &main_asset {
                        self.values.insert(market.B.clone(), value);
                    } else {
                        unreachable!();
                    }
                },
                Order::Update(market, value) => {
                    if &market.B == &main_asset {
                        self.values.insert(market.quote.clone(), Decimal::from(1) / value);
                    } else
                    if &market.quote == &main_asset {
                        self.values.insert(market.B.clone(), value);
                    } else {
                        unreachable!();
                    }
                }
            }

            let mut sum = Decimal::from(0);
            for (key, value) in &self.quantities {
                if let Some(v) = self.values.get(key) {
                    if value > &Decimal::from(0) {
                        println!("{} {} - {} {}", value, key, value * v, main_asset);
                        sum += value * v;
                    }
                }
            }
            println!("=============================================== TOTAL {} {}", sum, main_asset);
        }
    }
}
