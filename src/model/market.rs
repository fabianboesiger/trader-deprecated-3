use super::Asset;
use serde::Serialize;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize)]
pub struct Market {
    pub base: Asset,
    pub quote: Asset,
}

impl fmt::Display for Market {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.base, self.quote)
    }
}
