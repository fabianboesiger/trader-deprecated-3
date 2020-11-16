mod custom;

pub use custom::*;

use crate::{
    indicators::Indicator,
    model::Action,
};

pub trait Strategy<I: Indicator>: Clone + Send + 'static {
    fn run(&mut self, analysis: Option<I::Analysis>) -> Action;
}