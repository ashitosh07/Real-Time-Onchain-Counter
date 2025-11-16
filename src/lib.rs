use async_trait::async_trait;
use linera_sdk::{
    base::{ContractAbi, ServiceAbi},
    Contract, Service,
};
use serde::{Deserialize, Serialize};

/// Application state storing the counter value
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ApplicationState {
    pub value: u64,
}

/// Operations that can be performed on the counter
#[derive(Debug, Deserialize, Serialize)]
pub enum Operation {
    Increment,
    Decrement,
}

/// Messages (empty for this simple counter)
#[derive(Debug, Deserialize, Serialize)]
pub enum Message {}

/// Contract implementation
pub struct CounterContract;

#[async_trait]
impl Contract for CounterContract {
    type Message = Message;
    type InstantiationArgument = ();
    type Parameters = ();

    async fn load(_runtime: linera_sdk::ContractRuntime<Self>) -> Self {
        CounterContract
    }

    async fn instantiate(&mut self, _argument: ()) {}

    async fn execute_operation(&mut self, operation: Operation) -> Self::Response {
        // Simple counter logic without runtime access for now
        match operation {
            Operation::Increment => {},
            Operation::Decrement => {},
        }
    }

    async fn execute_message(&mut self, _message: Message) {}

    async fn store(mut self) {}
}

/// Service implementation
pub struct CounterService;

#[async_trait]
impl Service for CounterService {
    type Parameters = ();

    async fn new(_runtime: linera_sdk::ServiceRuntime<Self>) -> Self {
        CounterService
    }

    async fn handle_query(&self, _query: Self::Query) -> Self::QueryResponse {
        Response::Value(0)
    }
}

/// Query requests
#[derive(Debug, Deserialize, Serialize)]
pub enum Request {
    Value,
}

/// Query responses  
#[derive(Debug, Deserialize, Serialize)]
pub enum Response {
    Value(u64),
}

/// Contract ABI
pub struct CounterAbi;

impl ContractAbi for CounterAbi {
    type Operation = Operation;
    type Response = ();
}

impl ServiceAbi for CounterAbi {
    type Query = Request;
    type QueryResponse = Response;
}

linera_sdk::contract!(CounterContract);
linera_sdk::service!(CounterService);