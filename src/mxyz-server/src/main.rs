extern crate futures;
extern crate rocket;
extern crate tokio;

use mxyz_server::server::rocket_server::RocketServer;

#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    let server = RocketServer::new();
    server.init().await
}
