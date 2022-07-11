use crate::establish_connection;
use crate::schema::states;
use crate::schema::states::dsl::*;
use diesel::prelude::*;

#[derive(Insertable, Debug)]
#[table_name = "states"]
pub struct NewState<'a> {
    pub engine_id: &'a i32,
    pub state_id: &'a i32,
}

#[derive(Queryable, Debug)]
pub struct State {
    pub dbentry_id: i32,
    pub engine_id: i32,
    pub state_id: i32,
}
// impl std::convert::Into<mxyz_universe::state::State> for State {
//     fn into(self) -> mxyz_universe::state::State {
//         let other_state_id = self.state_id as usize;
//         let mut state = mxyz_universe::state::State::new(other_state_id);
//         state.systems = crate::system::get_systems(self.engine_id, self.state_id);
//         state
//     }
// }
impl std::convert::Into<mxyz_universe::state::SizedState> for State {
    fn into(self) -> mxyz_universe::state::SizedState {
        let other_state_id = self.state_id as usize;
        let mut state = mxyz_universe::state::SizedState::new(other_state_id);
        state.systems = crate::system::get_systems(self.engine_id, self.state_id);
        state
    }
}

// ============================================================================

pub fn get_db_states(engine_query: i32, state_query: &StateQuery) -> Vec<State> {
    let connection = crate::establish_connection();
    match state_query {
        StateQuery::Since(id) => states
            .filter(engine_id.eq(&engine_query))
            .filter(state_id.ge(id))
            .load::<State>(&connection)
            .expect("Error loading states"),
        StateQuery::Between(from, to) => states
            .filter(engine_id.eq(&engine_query))
            .filter(state_id.ge(from))
            .filter(state_id.lt(to))
            .load::<State>(&connection)
            .expect("Error loading states"),
        StateQuery::FromIds(_ids) => todo!("db-states from state-id list"),
    }
}

pub fn get_states(
    engine_query: i32,
    state_query: &StateQuery,
) -> Vec<mxyz_universe::state::SizedState> {
    let db_states = get_db_states(engine_query, &state_query);
    db_states
        .into_iter()
        .map(|db_state| {
            // println!("{:?}", db_state);
            db_state.into()
        })
        .collect()
}

// ============================================================================

#[derive(Debug)]
pub enum StateQuery {
    Since(i32),
    Between(i32, i32),
    FromIds(Vec<i32>),
}
