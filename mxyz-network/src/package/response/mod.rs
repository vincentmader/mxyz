use mxyz_universe::state::State;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Empty,
    StateVector(usize, Vec<State>), // engine-id, state-vec
    AddedEngine(usize),
    AddedClient(usize),
}
