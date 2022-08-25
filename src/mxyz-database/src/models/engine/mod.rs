use crate::establish_connection;
use crate::schema::engines;
use crate::schema::engines::dsl::*;
use diesel::prelude::*;

// ============================================================================

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

pub fn get_db_engines() -> Vec<Engine> {
    let connection = crate::establish_connection();
    engines
        .load::<Engine>(&connection)
        .expect("Error loading states")
}

pub fn create_engine<'a>(conn: &PgConnection, other_client_id: usize) -> Engine {
    // Determine new engine ID.
    let other_engine_id = std::cmp::max(1, get_db_engines().len() + 1);

    // Create new Database entry.
    let new_post = NewEngine {
        engine_id: &(other_engine_id as i32),
        client_id: &(other_client_id as i32),
    };

    diesel::insert_into(engines::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

// ============================================================================
