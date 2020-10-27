use super::{Asset, Value};

pub enum Action<B: Asset, Q: Asset> {
    Enter {
        stop_loss: Option<Value<B, Q>>,
        take_profit: Option<Value<B, Q>>,
    },
    Exit,
    Hold,
}

pub struct Update<B: Asset, Q: Asset> {
    value: Value<B, Q>,
    action: Action<B, Q>,
}