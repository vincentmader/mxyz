use std::collections::HashMap;

extern crate rocket_dyn_templates;
use rocket::Request;
use rocket_dyn_templates::Template;

#[catch(404)]
pub fn route(request: &Request<'_>) -> Template {
    let info_text = match request.format() {
        Some(ref mt) if !(mt.is_xml() || mt.is_html()) => {
            format!("'{}' requests are not supported.", mt)
        }
        _ => format!("Sorry, '{}' is an invalid path!", request.uri()),
    };
    let context: HashMap<&str, &str> = [("info_text", info_text.as_str())]
        .iter()
        .cloned()
        .collect();
    Template::render("error/404", &context)
}
