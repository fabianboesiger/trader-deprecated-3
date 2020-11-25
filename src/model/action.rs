use super::{Asset, Value};
use rust_decimal::Decimal;

#[derive(Debug)]
pub enum Action {
    Enter {
        stop_loss: Option<Decimal>,
        take_profit: Option<Decimal>,
        stake: Decimal,
    },
    Exit,
    Hold,
}
