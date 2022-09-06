use crate::state::UnsizedState;
use crate::system::sized_system::SizedSystem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)] // TODO remove clone again
pub struct SizedState {
    pub state_id: usize,
    pub systems: Vec<SizedSystem>,
}
impl SizedState {
    pub fn new(state_id: usize) -> Self {
        let systems = vec![];
        SizedState { state_id, systems }
    }
}
impl From<&UnsizedState> for SizedState {
    fn from(state: &UnsizedState) -> Self {
        let state_id = state.state_id;
        let systems = state.systems.iter().map(|sys| sys.into()).collect();
        SizedState { state_id, systems }
    }
}
