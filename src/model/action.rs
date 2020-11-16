use super::{Asset, Value};

pub enum Action {
    Enter {
        stop_loss: Option<f64>,
        take_profit: Option<f64>,
    },
    Exit,
    Hold,
}
