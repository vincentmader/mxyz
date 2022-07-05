use crate::schema::systems;
use mxyz_universe::system::planets::Planets;
use mxyz_universe::system::SystemVariant;

#[derive(Insertable, Debug)]
#[table_name = "systems"]
pub struct NewSystem<'a> {
    pub engine_id: &'a i32,
    pub state_id: &'a i32,
    pub system_id: &'a i32,
    pub system_variant_id: &'a i32,
}

#[derive(Queryable, Debug)]
pub struct System {
    pub dbentry_id: i32,
    pub engine_id: i32,
    pub state_id: i32,
    pub system_id: i32,
    pub system_variant_id: i32,
}
impl std::convert::Into<mxyz_universe::system::System> for System {
    fn into(self) -> mxyz_universe::system::System {
        let other_system_id = self.system_id as usize;
        let other_system_variant = SystemVariant::from(self.system_variant_id as usize);
        let other_system_variant = match other_system_variant {
            SystemVariant::Planets(_) => {
                let mut system = Planets::new();
                system.entities = crate::models::planet::get_planets(
                    self.engine_id,
                    self.state_id,
                    self.system_id,
                );
                SystemVariant::Planets(system)
            }
            _ => todo!(),
        };
        let system = mxyz_universe::system::System::new(other_system_id, other_system_variant);
        system
    }
}

// ============================================================================

use crate::establish_connection;
use crate::schema::systems::dsl::*;
use diesel::prelude::*;

pub fn get_db_systems(engine_query: i32, state_query: i32) -> Vec<System> {
    let db_conn = crate::establish_connection();
    systems
        .filter(engine_id.eq(&engine_query))
        .filter(state_id.eq(state_query))
        .load::<System>(&db_conn)
        .expect("Error loading systems")
}

pub fn get_systems(engine_query: i32, state_query: i32) -> Vec<mxyz_universe::system::System> {
    let db_systems = get_db_systems(engine_query, state_query);
    // println!("{:?}", db_systems);
    db_systems
        .into_iter()
        .map(|db_system| db_system.into())
        .collect()
}

pub fn create_system<'a>(conn: &PgConnection, system: mxyz_universe::system::System) -> System {
    let other_engine_id = 0; // DODO
    let other_state_id = 0; // TODO
    let other_system_id = system.system_id;
    let other_system_variant_id = 0; // TODO

    let new_system = NewSystem {
        engine_id: &(other_engine_id as i32),
        state_id: &(other_state_id as i32),
        system_id: &(other_system_id as i32),
        system_variant_id: &(other_system_variant_id as i32),
    };

    diesel::insert_into(systems::table)
        .values(&new_system)
        .get_result(conn)
        .expect("Error saving new post")
}
