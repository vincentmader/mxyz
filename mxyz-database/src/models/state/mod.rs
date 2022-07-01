use crate::schema::states;

#[derive(Queryable, Debug)]
pub struct State {
    pub dbentry_id: i32,
    pub state_id: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "states"]
pub struct NewState<'a> {
    pub state_id: &'a i32,
}
