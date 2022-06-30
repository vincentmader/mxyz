#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use self::diesel::prelude::*;
use self::models::*;
use self::models::{NewPlanet, Planet};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use schema::planets;
use schema::planets::dsl::*;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
    // diesel::delete(planets.filter(true)).execute(&conn)?;
    conn
}

fn show_planets() -> Vec<Planet> {
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

pub fn create_planet<'a>(
    conn: &PgConnection,
    new_planet: NewPlanet,
    // planet: mxyz_universe::entity::object::planet::Planet,
) -> Planet {
    // println!("{:?}", new_planet);
    diesel::insert_into(planets::table)
        .values(&new_planet)
        .get_result(conn)
        .expect("Error saving new post")
}
