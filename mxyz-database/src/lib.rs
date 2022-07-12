#![allow(unused_imports)]
#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use models::entity_v1::{EntityV1, NewEntityV1};
use models::state::{NewState, State};
use models::system::{NewSystem, System};
use models::*;
use schema::entities_v1;
use schema::entities_v1::dsl::*;
use schema::states;
use schema::states::dsl::*;
use schema::systems;
use schema::systems::dsl::*;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
    // diesel::delete(planets.filter(true)).execute(&conn)?;
    conn
}
