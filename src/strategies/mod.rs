mod custom;

pub use custom::*;

use crate::{
    indicators::Indicator,
    model::{Side, Action},
    trader::Position,
};

pub trait Strategy<I: Indicator>: Clone + Send + 'static {
    fn run(&mut self, analysis: Option<I::Analysis>, position: Position) -> Action;
}