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
// impl<'a> std::convert::From<&'a mxyz_engine::Engine> for NewEngine<'a> {
//     fn from(engine: &'a mxyz_engine::Engine) -> Self {
//         NewEngine {
//             client_id: &(engine.client_id as i32),
//             engine_id: &(engine.engine_id as i32),
//         }
//     }
// }

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

pub fn create_engine<'a>(conn: &PgConnection, engine: NewEngine) -> Engine {
    let other_client_id = crate::models::client::get_db_clients().len() - 1;
    let other_engine_id = engine.engine_id;

    let new_post = NewEngine {
        engine_id: &(*other_engine_id as i32),
        client_id: &(other_client_id as i32),
    };

    diesel::insert_into(engines::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}
