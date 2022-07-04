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
// impl std::convert::From<mxyz_universe::system::System> for NewSystem<'_> {
//     fn from(system: mxyz_universe::system::System) -> Self {
//         let other_client_id = 0; // TODO
//         let other_state_id = 0; // TODO
//         let other_system_id = system.system_id;
//         let other_entity_variant_id = 0; // TODO
//         NewSystem {
//             client_id: &(other_client_id as i32),
//             state_id: &(other_state_id as i32),
//             system_id: &(other_system_id as i32),
//             entity_variant_id: &(other_entity_variant_id as i32),
//         }
//     }
// }

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

pub fn create_system<'a>(conn: &PgConnection, system: mxyz_universe::system::System) -> System {
    let other_client_id = system.system_id; // TODO
    let other_state_id = 0; // TODO
    let other_system_id = system.system_id;
    let other_entity_variant_id = 0; // TODO

    let new_post = NewSystem {
        // system_id: &0,
        // entities: vec![],
        client_id: &(other_client_id as i32),
        state_id: &(other_state_id as i32),
        system_id: &(other_system_id as i32),
        entity_variant_id: &(other_entity_variant_id as i32),
    };

    diesel::insert_into(systems::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}
