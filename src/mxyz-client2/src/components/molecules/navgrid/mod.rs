pub mod navgrid_cell;
pub mod navgrid_section;
pub mod navgrid_thumbnail;
use mxyz_engine::config::simulation_variant::PhysicalField;
// use mxyz_engine::config::simulation_variant::SimulationVariant;
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("../../../../../mxyz-server/static/css/index/navgrid.css");

#[function_component(Navgrid)]
pub fn fn_name() -> Html {
    let style = stylist::Style::new(STYLE_FILE).unwrap();
    html! {
        <div class={style}>
            <div class="navgrid_container">
                {
                    PhysicalField::get_all().iter().map(|field| {
                        html!{<navgrid_section::Section field={field.clone()} />
                    }}).collect::<Html>()
                }
            </div>
        </div>
    }
}

#[derive(PartialEq, yew::Properties, std::default::Default)]
pub struct Props {}
