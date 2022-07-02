use crate::schema::planets;

#[derive(Queryable, Debug)]
pub struct Planet {
    pub dbentry_id: i32,
    pub client_id: i32,
    pub engine_id: i32,
    pub state_id: i32,
    pub system_id: i32,
    pub planet_id: i32,
    pub mass: f64,
    pub pos_x: f64,
    pub pos_y: f64,
    pub pos_z: f64,
    pub vel_x: f64,
    pub vel_y: f64,
    pub vel_z: f64,
}

#[derive(Insertable, Debug)]
#[table_name = "planets"]
pub struct NewPlanet<'a> {
    pub engine_id: &'a i32,
    pub client_id: &'a i32,
    pub state_id: &'a i32,
    pub system_id: &'a i32,
    pub planet_id: &'a i32,
    pub mass: &'a f64,
    pub pos_x: &'a f64,
    pub pos_y: &'a f64,
    pub pos_z: &'a f64,
    pub vel_x: &'a f64,
    pub vel_y: &'a f64,
    pub vel_z: &'a f64,
}

// ============================================================================

use crate::establish_connection;
use crate::schema::planets::dsl::*;
use diesel::prelude::*;

pub fn get_db_planets(engine_query: i32, state_query: i32, system_query: i32) -> Vec<Planet> {
    let connection = crate::establish_connection();
    planets
        .filter(engine_id.eq(&engine_query))
        .filter(state_id.eq(state_query))
        .filter(system_id.eq(system_query))
        .load::<Planet>(&connection)
        .expect("Error loading systems")
}