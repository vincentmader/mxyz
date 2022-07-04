use mxyz_engine::state::State;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Empty,
    StateVector(Vec<State>),
    AddedEngine,
    AddedClient(usize),
}
