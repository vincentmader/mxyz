pub mod error;
pub mod index;
pub mod simulation;

use rocket::response::content;
use rocket::{get, routes};

/// Returns Vector of all Rocket Routes
pub fn get_all_routes() -> Vec<rocket::Route> {
    routes![
        index::route,
        simulation::route,
        // // testing
        test_json,
        // test_bytes,
        // test_tcp,
        // test_download,
        test_db,
        test_engine,
    ]
}

// use mxyz_engine::state::State;
// use rocket_contrib::json::Json;

// #[get("/state/<id>")]
// fn test_get_state(id: usize) -> Json<State> {
//     // let user_from_id = User::from(id);
//     /* ... */
//     let state = State::new();
//     Json(state)
// }

// ============================================================================

#[get("/test_engine")]
fn test_engine() -> content::Json<&'static str> {
    todo!();
    // let foo = "{ 'hi': 'world' }"; // TODO get from engine's state-vec (or db?)
    // content::Json(foo)
}

// WORKS! =====================================================================
#[get("/test_json")]
fn test_json() -> content::Json<&'static str> {
    let foo = "{ 'hi': 'world' }"; // TODO get from engine's state-vec (or db?)
    content::Json(foo)
}

// WORKS! =====================================================================
#[get("/test_download")]
fn _test_download() -> Vec<u8> {
    let bytes = vec![0, 1, 2];
    bytes
}

// WORKS! =====================================================================
use diesel::prelude::*;
// use mxyz_database::models::*;
use mxyz_database::models::{NewPlanet, Planet};
use mxyz_database::schema::planets;
use mxyz_database::schema::planets::dsl::*;
// use mxyz_database::*;
// use mxyz_database::schema::*;
// use self::diesel::prelude::*;
// use self::diesel_demo::*;

#[get("/test_db")]
// fn test_bytes() -> content::Json<&'static str> {
fn test_db() -> content::Json<&'static str> {
    let connection = mxyz_database::establish_connection();

    // show
    let results = planets
        // .filter(published.eq(true))
        // .limit(5)
        .load::<Planet>(&connection)
        .expect("Error loading posts");
    let nr_of_planets = results.len();
    println!("Displaying {} planets", nr_of_planets);
    for planet in results {
        println!("{}", planet.planet_id);
        println!("----------\n");
        println!("{}", planet.mass);
    }

    // add
    let new_planet = NewPlanet {
        planet_id: &((nr_of_planets + 1) as i32),
        system_id: &0,
        mass: &1.,
        pos_x: &0.,
        pos_y: &0.,
        pos_z: &0.,
        vel_x: &0.,
        vel_y: &0.,
        vel_z: &0.,
    };
    let _planet: Planet = diesel::insert_into(planets::table)
        .values(&new_planet)
        .get_result(&connection)
        .expect("Error saving new post");

    let foo = "{ 'hello': 'world' }"; // TODO get from engine's state-vec (or db?)
    content::Json(foo)
}

// ==== ? =====================================================================
#[get("/test_bytes")]
// fn test_bytes() -> content::Json<&'static str> {
fn _test_bytes() -> Vec<u8> {
    let bytes = vec![0, 1, 2];
    bytes
}

// ==== ? =====================================================================
#[get("/test_tcp")]
fn _test_tcp() -> content::Json<&'static str> {
    _tcp().unwrap();
    let foo = "{}"; // TODO bytes!
    content::Json(foo)
}

use std::net::{TcpListener, TcpStream};
fn _handle_client(_stream: TcpStream) {
    // ...
}

fn _tcp() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;
    // accept connections and process them serially
    for stream in listener.incoming() {
        _handle_client(stream?);
    }
    Ok(())
}
