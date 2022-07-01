use crate::schema::systems;

#[derive(Queryable, Debug)]
pub struct System {
    pub dbentry_id: i32,
    pub system_id: i32,
    pub state_id: i32,
    pub entity_variant_id: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "systems"]
pub struct NewSystem<'a> {
    pub system_id: &'a i32,
    pub state_id: &'a i32,
    pub entity_variant_id: &'a i32,
}
