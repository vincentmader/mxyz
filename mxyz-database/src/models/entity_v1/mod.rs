use crate::establish_connection;
use crate::schema::entities_v1;
use crate::schema::entities_v1::dsl::*;
use diesel::prelude::*;

#[derive(Insertable, Debug)]
#[table_name = "entities_v1"]
pub struct NewEntityV1<'a> {
    pub engine_id: &'a i32,
    pub state_id: &'a i32,
    pub system_id: &'a i32,
    pub entity_id: &'a i32,
    pub mass: &'a f64,
    pub pos_x: &'a f64,
    pub pos_y: &'a f64,
    pub pos_z: &'a f64,
    pub vel_x: &'a f64,
    pub vel_y: &'a f64,
    pub vel_z: &'a f64,
}

#[derive(Queryable, Debug)]
pub struct EntityV1 {
    pub dbentry_id: i32,
    pub engine_id: i32,
    pub state_id: i32,
    pub system_id: i32,
    pub entity_id: i32,
    pub mass: f64,
    pub pos_x: f64,
    pub pos_y: f64,
    pub pos_z: f64,
    pub vel_x: f64,
    pub vel_y: f64,
    pub vel_z: f64,
}
impl std::convert::Into<mxyz_universe::entity::EntityV1> for EntityV1 {
    fn into(self) -> mxyz_universe::entity::EntityV1 {
        mxyz_universe::entity::EntityV1 {
            mass: self.mass,
            position: [self.pos_x, self.pos_y, self.pos_z],
            velocity: [self.vel_x, self.vel_y, self.vel_z],
        }
    }
}

// ============================================================================

pub fn get_db_entities(engine_query: i32, state_query: i32, system_query: i32) -> Vec<EntityV1> {
    let connection = crate::establish_connection();
    entities_v1
        .filter(engine_id.eq(&engine_query))
        .filter(state_id.eq(state_query))
        .filter(system_id.eq(system_query))
        .load::<EntityV1>(&connection)
        .expect("Error loading systems")
}

pub fn get_entities(
    engine_query: i32,
    state_query: i32,
    system_query: i32,
) -> Vec<mxyz_universe::entity::EntityV1> {
    let db_entities = get_db_entities(engine_query, state_query, system_query);
    // println!("{:?}", db_planets);
    db_entities
        .into_iter()
        .map(|db_planet| db_planet.into())
        .collect()
}
