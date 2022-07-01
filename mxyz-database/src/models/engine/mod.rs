use crate::schema::engines;

#[derive(Queryable, Debug)]
pub struct Engine {
    pub dbentry_id: i32,
    pub client_id: i32,
    pub engine_id: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "engines"]
pub struct NewEngine<'a> {
    pub client_id: &'a i32,
    pub engine_id: &'a i32,
}

// ============================================================================

use crate::establish_connection;
use crate::schema::engines::dsl::*;
use diesel::prelude::*;

pub fn get_db_engines() -> Vec<Engine> {
    let connection = crate::establish_connection();
    engines
        .load::<Engine>(&connection)
        .expect("Error loading states")
}
