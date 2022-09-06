use crate::establish_connection;
use crate::schema::states;
use crate::schema::states::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use mxyz_engine::state::StateQuery;

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
impl std::convert::Into<mxyz_engine::state::SizedState> for State {
    fn into(self) -> mxyz_engine::state::SizedState {
        let other_state_id = self.state_id as usize;
        let mut state = mxyz_engine::state::SizedState::new(other_state_id);
        let conn = crate::establish_connection();
        state.systems = crate::system::get_systems(&conn, self.engine_id, self.state_id);
        state
    }
}

// ============================================================================

pub fn create_state<'a>(conn: &PgConnection, new_state: NewState) -> State {
    diesel::insert_into(states::table)
        .values(&new_state)
        .get_result(conn)
        .expect("Error saving new state")
}

pub fn get_db_states(
    conn: &PgConnection,
    engine_query: i32,
    state_query: &StateQuery,
) -> Vec<State> {
    match state_query {
        // Get all states since a given state-id.
        StateQuery::BatchSince(batch_size, last_sync) => states
            .filter(engine_id.eq(&engine_query))
            .filter(state_id.gt(*last_sync as i32))
            .filter(state_id.le((*last_sync + *batch_size + 1) as i32)) // +1 needed?
            .load::<State>(conn)
            .expect("Error loading states"),

        // Get all states since a given state-id.
        StateQuery::AllSince(last_sync) => states
            .filter(engine_id.eq(&engine_query))
            .filter(state_id.gt(*last_sync as i32))
            .load::<State>(conn)
            .expect("Error loading states"),

        // Get all states between two state-ids.
        StateQuery::Between(from, to) => states
            .filter(engine_id.eq(&engine_query))
            .filter(state_id.ge(*from as i32))
            .filter(state_id.lt(*to as i32))
            .load::<State>(conn)
            .expect("Error loading states"),
        // TODO Get all states from list of state-ids.
        // StateQuery::FromIds(_ids) => {}
        // TODO Get batch of newest states.
    }
}

pub fn get_states(
    conn: &PgConnection,
    engine_query: i32,
    state_query: &StateQuery,
) -> Vec<mxyz_engine::state::SizedState> {
    let db_states = get_db_states(conn, engine_query, &state_query);
    // std::fs::write("../log/got_state_from_db.txt", "").unwrap();
    db_states
        .into_iter()
        .map(|db_state| db_state.into())
        .collect()
}
