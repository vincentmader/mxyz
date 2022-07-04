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
pub fn create_client<'a>(conn: &PgConnection, other_client_id: usize) -> Client {
    let new_post = NewClient {
        client_id: &(other_client_id as i32),
    };

    diesel::insert_into(clients::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}
