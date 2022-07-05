use crate::schema::states;

#[derive(Queryable, Debug)]
pub struct State {
    pub dbentry_id: i32,
    pub client_id: i32,
    pub engine_id: i32,
    pub state_id: i32,
}
impl std::convert::Into<mxyz_engine::state::State> for State {
    fn into(self) -> mxyz_engine::state::State {
        let mut state = mxyz_engine::state::State::new();
        state.state_id = self.state_id as usize;
        state.systems = crate::system::get_systems(self.client_id, self.engine_id);
        // println!("{:?}", state.systems);
        state
    }
}

#[derive(Insertable, Debug)]
#[table_name = "states"]
pub struct NewState<'a> {
    pub client_id: &'a i32,
    pub engine_id: &'a i32,
    pub state_id: &'a i32,
}

pub enum StateQuery {
    Since(i32),
    Between(i32, i32),
    FromIds(Vec<i32>),
}

// ============================================================================

use crate::establish_connection;
use crate::schema::states::dsl::*;
use diesel::prelude::*;

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
pub fn get_states(engine_query: i32, state_query: &StateQuery) -> Vec<mxyz_engine::state::State> {
    let db_states = get_db_states(engine_query, &state_query);
    db_states
        .into_iter()
        .map(|db_state| {
            println!("{:?}", db_state);
            db_state.into()
        })
        .collect()
}
