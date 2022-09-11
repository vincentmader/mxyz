use mxyz_engine::config::EngineConfig;
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
    /// - engine-config
    /// - state-vector
    StateVector(usize, StateQuery, EngineConfig, Vec<SizedState>),

    Empty,
}
