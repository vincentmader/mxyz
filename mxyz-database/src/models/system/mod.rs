use crate::schema::systems;

#[derive(Queryable, Debug)]
pub struct System {
    pub dbentry_id: i32,
    pub client_id: i32,
    pub engine_id: i32,
    pub state_id: i32,
    pub system_id: i32,
    pub entity_variant_id: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "systems"]
pub struct NewSystem<'a> {
    pub client_id: &'a i32,
    pub state_id: &'a i32,
    pub system_id: &'a i32,
    pub entity_variant_id: &'a i32,
}

// ============================================================================

use crate::establish_connection;
use crate::schema::systems::dsl::*;
use diesel::prelude::*;

pub fn get_db_systems(engine_query: i32, state_query: i32) -> Vec<System> {
    let connection = crate::establish_connection();
    systems
        .filter(engine_id.eq(&engine_query))
        .filter(state_id.eq(state_query))
        .load::<System>(&connection)
        .expect("Error loading systems")
}
