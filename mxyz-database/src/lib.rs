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

fn _show_entities_v1() -> Vec<EntityV1> {
    let connection = establish_connection();
    let results = entities_v1
        // .filter(published.eq(true))
        .limit(5)
        .load::<EntityV1>(&connection)
        .expect("Error loading planets");

    println!("Displaying {} entities", results.len());
    // for planet in results.iter() {
    // println!("{}", planet.mass);
    // }
    results
}

// TODO remove below !!!

pub fn create_planet<'a>(conn: &PgConnection, new_entity: NewEntityV1) -> EntityV1 {
    diesel::insert_into(entities_v1::table)
        .values(&new_entity)
        .get_result(conn)
        .expect("Error saving new planet")
}

pub fn create_system<'a>(conn: &PgConnection, new_system: NewSystem) -> System {
    diesel::insert_into(systems::table)
        .values(&new_system)
        .get_result(conn)
        .expect("Error saving new system")
}

pub fn create_state<'a>(conn: &PgConnection, new_state: NewState) -> State {
    diesel::insert_into(states::table)
        .values(&new_state)
        .get_result(conn)
        .expect("Error saving new state")
}
