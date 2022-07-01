#![allow(unused_imports)]
#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use models::planet::{NewPlanet, Planet};
use models::state::{NewState, State};
use models::system::{NewSystem, System};
use models::*;
use schema::planets;
use schema::planets::dsl::*;
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

fn _show_planets() -> Vec<Planet> {
    let connection = establish_connection();
    let results = planets
        // .filter(published.eq(true))
        .limit(5)
        .load::<Planet>(&connection)
        .expect("Error loading planets");

    println!("Displaying {} planets", results.len());
    for planet in results.iter() {
        println!("{}", planet.mass);
    }
    results
}

pub fn create_planet<'a>(conn: &PgConnection, new_planet: NewPlanet) -> Planet {
    diesel::insert_into(planets::table)
        .values(&new_planet)
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
