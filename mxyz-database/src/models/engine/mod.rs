use crate::schema::engines;

#[derive(Queryable, Debug)]
pub struct Engine {
    pub dbentry_id: i32,
    pub engine_id: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "engines"]
pub struct NewEngine<'a> {
    pub engine_id: &'a i32,
}
