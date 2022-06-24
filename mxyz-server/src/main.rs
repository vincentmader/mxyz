extern crate futures;
extern crate rocket;
extern crate tokio;
use mxyz_server::server::Server;

#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    let server = Server::new();
    server.start().await
}
