use mxyz_universe::state::SizedState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Empty,
    StateVector(usize, Vec<SizedState>), // engine-id, state-vec
    AddedEngine(usize),
    AddedClient(usize),
}
