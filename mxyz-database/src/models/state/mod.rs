use crate::schema::states;

#[derive(Queryable, Debug)]
pub struct State {
    pub dbentry_id: i32,
    pub client_id: i32,
    pub engine_id: i32,
    pub state_id: i32,
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

pub fn get_db_states(engine_query: i32, state_query: StateQuery) -> Vec<State> {
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
