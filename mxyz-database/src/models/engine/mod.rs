use crate::schema::engines;

#[derive(Queryable, Debug)]
pub struct Engine {
    pub dbentry_id: i32,
    pub client_id: i32,
    pub engine_id: i32,
}
// impl std::convert::From<mxyz_engine::Engine> for Engine {
//     fn from(engine: mxyz_engine::Engine) -> Self {
//         Engine {}
//     }
// }

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

pub fn create_engine<'a>(
    conn: &PgConnection,
    other_client_id: usize,
    // other_engine_id: usize,
) -> Engine {
    // let nr_of_clients = get_db_engines().len();
    // Determine ID of new Client.
    // - start counting at 1 (Diesel default)
    // - if e.g. 5 Engines in DB  ->  new ID: 6
    // let other_client_id = std::cmp::max(1, nr_of_clients + 1);

    // Get Nr. of Engines already in Database.
    let nr_of_engines_in_db = get_db_engines().len();
    // Determine new Engine-ID.
    let other_engine_id = std::cmp::max(1, nr_of_engines_in_db + 1);
    // if new_engine_id == other_engine_id {
    //     panic!("engine-id setup faulty");
    // }

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
