pub mod error;
pub mod index;
pub mod simulation;

use rocket::response::content;

/// Returns Vector of all Rocket Routes
pub fn get_all_routes() -> Vec<rocket::Route> {
    routes![
        index::route,
        simulation::route,
        // testing
        test_json,
        test_bytes,
        test_tcp,
        test_download,
        test_db,
        test_engine,
        // test_stream,
    ]
}

// #[options("/login")]
// pub fn handle_cors_request() {}

// #[post("/login", format = "json", data = "<credentials>")]
// pub fn login(credentials: Json<Credentials>) -> Json<bool> {
//     println!("{:?}", credentials);
//     let username = &credentials.username;
//     let password = &credentials.password;
//     if credentials.username == "a" {}
//     let valid = match (username.as_str(), password.as_str()) {
//         ("root", "root") => true,
//         ("vincent", "root") => true,
//         _ => false,
//     };
//     println!("valid {}", valid);
//     Json::from(valid)
// }

// ============================================================================

// use std::io::prelude::*;

// const HOST: &'static str = "http://127.0.0.1:8000";

// #[get("/proxy")]
// pub fn test_stream() -> std::io::Result<()> {
//     // let mut stream = TcpStream::connect(HOST)?;
//     let mut stream = TcpStream::connect("http://127.0.0.1:8000")?;

//     stream.write(&[1])?;
//     stream.read(&mut [0; 128])?;
//     Ok(())
// } // the stream is closed here

// ============================================================================

// use bytes::Bytes;
// use tokio_util::io::StreamReader;
// #[get("/proxy")]
// async fn proxify() -> rocket::response::Stream<
//     StreamReader<impl rocket::futures::Stream<Item = Result<Bytes, std::io::Error>>, Bytes>,
// > {
//     let url =
//         reqwest::Url::parse("https://www.rust-lang.org/static/images/rust-logo-blk.svg").unwrap();
//     let client = reqwest::Client::new();
//     let response = client.get(url).send().await.unwrap();

//     use rocket::futures::TryStreamExt; // for map_err() call below:
//     let reader = StreamReader::new(
//         response
//             .bytes_stream()
//             .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)),
//     );
//     rocket::response::Stream::chunked(reader, 4096)
// }

// ============================================================================

#[get("/test_engine")]
fn test_engine() -> content::Json<&'static str> {
    let foo = "{ 'hi': 'world' }"; // TODO get from engine's state-vec (or db?)
    content::Json(foo)
}

// WORKS! =====================================================================
#[get("/test_json")]
fn test_json() -> content::Json<&'static str> {
    let foo = "{ 'hi': 'world' }"; // TODO get from engine's state-vec (or db?)
    content::Json(foo)
}

// WORKS! =====================================================================
#[get("/test_download")]
fn test_download() -> Vec<u8> {
    let bytes = vec![0, 1, 2];
    bytes
}

// WORKS! =====================================================================
use diesel::prelude::*;
use mxyz_database::models::*;
use mxyz_database::models::{NewPlanet, Planet};
use mxyz_database::schema::planets;
use mxyz_database::schema::planets::dsl::*;
use mxyz_database::*;
// use mxyz_database::schema::*;
// use self::diesel::prelude::*;
// use self::diesel_demo::*;

#[get("/test_db")]
fn test_db() -> Vec<u8> {
    let connection = mxyz_database::establish_connection();

    // show
    let results = planets
        .limit(i64::MAX)
        .load::<Planet>(&connection)
        .expect("Error loading posts");
    let nr_of_planets = results.len();
    println!("Displaying {} planets", nr_of_planets);
    for planet in results.iter() {
        println!("{}", planet.planet_id);
        println!("----------\n");
        println!("{}", planet.mass);
    }

    // add
    let new_planet = NewPlanet {
        planet_id: &(nr_of_planets as i32),
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

    let mut bytes = vec![];
    for planet in results.iter() {
        // let foo = planet.
        bytes.push(0);
    }
    bytes
}

// ==== ? =====================================================================
#[get("/test_bytes")]
// fn test_bytes() -> content::Json<&'static str> {
fn test_bytes() -> Vec<u8> {
    let bytes = vec![0, 1, 2];
    bytes
}

// ==== ? =====================================================================
#[get("/test_tcp")]
fn test_tcp() -> content::Json<&'static str> {
    tcp().unwrap();
    let foo = "{}"; // TODO bytes!
    content::Json(foo)
}

use std::net::{TcpListener, TcpStream};
fn handle_client(stream: TcpStream) {
    // ...
}

fn tcp() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;
    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
