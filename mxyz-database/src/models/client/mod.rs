use crate::schema::clients;

#[derive(Queryable, Debug)]
pub struct Client {
    pub dbentry_id: i32,
    pub client_id: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "clients"]
pub struct NewClient<'a> {
    pub client_id: &'a i32,
}

// ============================================================================

use crate::establish_connection;
use crate::schema::clients::dsl::*;
use diesel::prelude::*;

pub fn get_db_clients() -> Vec<Client> {
    let connection = crate::establish_connection();

    clients
        .load::<Client>(&connection)
        .expect("Error loading states")
}
pub fn create_client<'a>(conn: &PgConnection) -> Client {
    // Get Nr of Clients already in Database.
    let nr_of_clients = get_db_clients().len();
    // Determine ID of new Client.
    // - start counting at 1 (Diesel default)
    // - if e.g. 5 Engines in DB  ->  new ID: 6
    let other_client_id = std::cmp::max(1, nr_of_clients + 1);
    // Create new DB Entry.
    let new_post = NewClient {
        client_id: &(other_client_id as i32),
    };
    diesel::insert_into(clients::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}
