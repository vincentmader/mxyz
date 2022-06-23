#[derive(Queryable, Debug)]
pub struct Planet {
    pub planet_id: i32,
    pub system_id: i32,
    pub mass: f64,
    pub pos_x: f64,
    pub pos_y: f64,
    pub pos_z: f64,
    pub vel_x: f64,
    pub vel_y: f64,
    pub vel_z: f64,
}

use super::schema::planets;

#[derive(Insertable, Debug)]
#[table_name = "planets"]
pub struct NewPlanet<'a> {
    pub planet_id: &'a i32,
    pub system_id: &'a i32,
    pub mass: &'a f64,
    pub pos_x: &'a f64,
    pub pos_y: &'a f64,
    pub pos_z: &'a f64,
    pub vel_x: &'a f64,
    pub vel_y: &'a f64,
    pub vel_z: &'a f64,
}