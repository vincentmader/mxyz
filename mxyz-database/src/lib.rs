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
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn show_planets() {
    let connection = establish_connection();
    let results = planets
        // .filter(published.eq(true))
        .limit(5)
        .load::<Planet>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for planet in results {
        println!("{}", planet.mass);
    }
}

pub fn create_planet<'a>(
    conn: &PgConnection,
    new_planet: NewPlanet,
    // planet: mxyz_universe::entity::object::planet::Planet,
) -> Planet {
    // let new_planet = NewPlanet {
    //     planet_id: &0,
    //     system_id: &0,
    //     mass: &1.,
    //     pos_x: &0.,
    //     pos_y: &0.,
    //     pos_z: &0.,
    //     vel_x: &0.,
    //     vel_y: &0.,
    //     vel_z: &0.,
    // };

    println!("{:?}", new_planet);
    diesel::insert_into(planets::table)
        .values(&new_planet)
        .get_result(conn)
        .expect("Error saving new post")
}
