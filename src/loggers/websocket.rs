use super::Logger;

pub struct Websocket;

impl Websocket {
    pub fn new() -> Self {
        Self {}
    }
}

impl Logger for Websocket {}
