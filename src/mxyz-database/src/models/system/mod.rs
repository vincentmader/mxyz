use crate::models::entity_v1;
use crate::schema::systems;
use diesel::pg::PgConnection;
use mxyz_engine::system::entities_v1::EntitiesV1;
use mxyz_engine::system::sized_system::sized_system_variant::SizedSystemVariant;
use mxyz_engine::system::sized_system::SizedSystem;
use mxyz_engine::system::unsized_system::unsized_system_variant::UnsizedSystemVariant;

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
impl System {
    fn into(self, conn: &PgConnection) -> SizedSystem {
        let other_system_id = self.system_id as usize;
        let other_system_variant = UnsizedSystemVariant::from(self.system_variant_id as usize);
        let other_system_variant = match other_system_variant {
            UnsizedSystemVariant::EntitiesV1 => {
                let entities =
                    entity_v1::get_entities(&conn, self.engine_id, self.state_id, self.system_id);
                //
                let mut system = EntitiesV1::new();
                system.entities = entities;
                SizedSystemVariant::EntitiesV1(system)
            }
            _ => todo!(),
        };
        let system = SizedSystem::new(other_system_id, other_system_variant);
        system
    }
}
// -----------------------------------------------------------------------------
use crate::establish_connection;
use crate::schema::systems::dsl::*;
use diesel::prelude::*;

pub fn get_db_systems(conn: &PgConnection, engine_query: i32, state_query: i32) -> Vec<System> {
    systems
        .filter(engine_id.eq(&engine_query))
        .filter(state_id.eq(state_query))
        .load::<System>(conn)
        .expect("Error loading systems")
}

pub fn get_systems(conn: &PgConnection, engine_query: i32, state_query: i32) -> Vec<SizedSystem> {
    let db_systems = get_db_systems(conn, engine_query, state_query);
    db_systems
        .into_iter()
        .map(|db_system| db_system.into(conn))
        .collect()
}

pub fn create_system<'a>(conn: &PgConnection, new_system: NewSystem) -> System {
    diesel::insert_into(systems::table)
        .values(&new_system)
        .get_result(conn)
        .expect("Error saving new system")
}
