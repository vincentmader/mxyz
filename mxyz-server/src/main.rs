extern crate futures;
extern crate rocket;
extern crate tokio;
use mxyz_server::server::RocketServer;

#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    let server = RocketServer::new();
    server.start().await
}
