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
