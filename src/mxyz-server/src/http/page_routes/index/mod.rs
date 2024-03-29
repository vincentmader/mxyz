use std::collections::HashMap;

extern crate rocket_dyn_templates;
use rocket::get;
use rocket_dyn_templates::Template;

mod navgrid;
use navgrid::NavGrid;

#[get("/")]
pub fn route() -> Template {
    let navgrid = NavGrid::new();
    let context: HashMap<&str, &NavGrid> = [("navgrid", &navgrid)].iter().cloned().collect();
    // Template::render("yew_index", &context)
    Template::render("tera/index/base", &context)
}
