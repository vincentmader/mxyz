use super::schema::planets;
use super::schema::states;
use super::schema::systems;

// ============================================================================

#[derive(Queryable, Debug)]
pub struct Planet {
    pub dbentry_id: i32,
    pub step_id: i32,
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
    pub step_id: &'a i32,
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

#[derive(Queryable, Debug)]
pub struct State {
    pub dbentry_id: i32,
    pub state_id: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "states"]
pub struct NewState<'a> {
    pub state_id: &'a i32,
}

// ============================================================================

#[derive(Queryable, Debug)]
pub struct System {
    pub dbentry_id: i32,
    pub system_id: i32,
    pub state_id: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "systems"]
pub struct NewSystem<'a> {
    pub system_id: &'a i32,
    pub state_id: &'a i32,
}
