use mxyz_engine::state::State;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    StateVector(Vec<State>),
}
