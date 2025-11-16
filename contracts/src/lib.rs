use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Operation {
    Increment,
    Decrement,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ApplicationState {
    pub value: i64,
}

pub struct CounterContract;

impl CounterContract {
    pub fn new() -> Self {
        CounterContract
    }

    pub fn increment(&mut self, state: &mut ApplicationState) {
        state.value += 1;
    }

    pub fn decrement(&mut self, state: &mut ApplicationState) {
        state.value -= 1;
    }

    pub fn get_value(&self, state: &ApplicationState) -> i64 {
        state.value
    }
}