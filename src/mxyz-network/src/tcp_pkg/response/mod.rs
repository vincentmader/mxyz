use mxyz_engine::state::SizedState;
use mxyz_engine::state::StateQuery;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Empty,
    StateVector(usize, StateQuery, Vec<SizedState>), // engine-id, query, state-vec
    AddedEngine(usize),
    AddedClient(usize),
}
