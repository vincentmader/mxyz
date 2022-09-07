use mxyz_engine::state::SizedState;
use mxyz_engine::state::StateQuery;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    /// Response: Client was added.
    /// - client-id
    AddedClient(usize),

    /// Response: Engine was added.
    /// - engine-id
    AddedEngine(usize),

    /// Response: State Vector.
    /// - engine-id
    /// - state-query
    /// - state-vector
    StateVector(usize, StateQuery, Vec<SizedState>),

    Empty,
}
