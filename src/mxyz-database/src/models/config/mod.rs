use crate::models::entity_v1;
use crate::schema::configs;
use crate::schema::configs::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use mxyz_engine::config::EngineConfig;
use mxyz_engine::system::entities_v1::EntitiesV1;
use mxyz_engine::system::sized_system::sized_system_variant::SizedSystemVariant;
use mxyz_engine::system::sized_system::SizedSystem;
use mxyz_engine::system::unsized_system::unsized_system_variant::UnsizedSystemVariant;

#[derive(Insertable, Debug)]
#[table_name = "configs"]
pub struct NewConfig<'a> {
    pub engine_id: &'a i32,
}

#[derive(Queryable, Debug)]
pub struct Config {
    pub dbentry_id: i32,
    pub engine_id: i32,
}

pub fn get_db_config(conn: &PgConnection, engine_query: i32) -> Vec<Config> {
    configs
        .filter(engine_id.eq(&engine_query))
        .load::<Config>(conn)
        .expect("Error loading systems")
}

pub fn get_config(conn: &PgConnection, engine_query: i32) -> EngineConfig {
    // let db_config = &get_db_config(conn, engine_query)[0];
    // TODO get config somehow (?)
    EngineConfig::new()
}

pub fn create_config<'a>(conn: &PgConnection, new_config: NewConfig) -> Config {
    diesel::insert_into(configs::table)
        .values(&new_config)
        .get_result(conn)
        .expect("Error saving new system")
}
