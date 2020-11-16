use super::Series;

#[derive(Clone)]
pub struct Value;

impl Series for Value {
    type Analysis = f64;

    fn new() -> Self {
        Value
    }

    fn compute(&mut self, value: f64, _recover: bool) -> Option<Self::Analysis> {
        Some(value)
    }
}
