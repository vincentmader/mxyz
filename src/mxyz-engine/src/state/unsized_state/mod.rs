use crate::state::SizedState;
use crate::system::System;

/// State
#[derive(Debug)]
pub struct UnsizedState {
    pub state_id: usize,
    pub systems: Vec<System>, // todo include into db ?
}
impl UnsizedState {
    /// Creates new instance of State Structure
    pub fn new(state_id: usize) -> Self {
        let systems = vec![];
        UnsizedState { state_id, systems }
    }
}
impl From<&SizedState> for UnsizedState {
    fn from(state: &SizedState) -> Self {
        let state_id = state.state_id;
        let systems = state.systems.iter().map(|sys| sys.into()).collect();
        UnsizedState { state_id, systems }
    }
}
