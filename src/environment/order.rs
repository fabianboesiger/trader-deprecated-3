use super::Trader;
use rust_decimal::Decimal;

#[derive(Debug)]
pub enum Order {
    Buy(MarketSymbol, Decimal),
    Sell(MarketSymbol, Decimal),
    Update(MarketSymbol, Decimal),
}