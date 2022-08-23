use mxyz_universe::state::SizedState;
use mxyz_universe::state::StateQuery;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Empty,
    StateVector(usize, StateQuery, Vec<SizedState>), // engine-id, query, state-vec
    AddedEngine(usize),
    AddedClient(usize),
}
