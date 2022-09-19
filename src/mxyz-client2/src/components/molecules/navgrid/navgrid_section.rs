use super::navgrid_cell::NavgridCell;
use crate::components::app::AppPage;
use mxyz_engine::config::simulation_variant::PhysicalField;
use mxyz_engine::config::simulation_variant::SimulationVariant;
use yew::prelude::*;

const STYLE_FILE: &str =
    include_str!("../../../../../mxyz-server/static/css/index/navgrid_section.css");

#[derive(Properties, PartialEq)]
pub struct Props {
    pub field: PhysicalField,
    pub on_page_change: Callback<AppPage>,
}

#[function_component(Section)]
pub fn fn_name(props: &Props) -> Html {
    let style = stylist::Style::new(STYLE_FILE).unwrap();
    let field = props.field.clone();
    let simulations: Vec<SimulationVariant> = SimulationVariant::get_by_physical_field(field);
    let on_page_change = props.on_page_change.clone();
    html! {
        <div class={style}>
            <div class="navgrid_section_title">{props.field.to_string()}</div>
            <div class="navgrid_section">
                <div class="navgrid">
                    {
                        simulations.iter().map(|variant| {
                            html!{<NavgridCell simulation_variant={variant.clone()} on_page_change={on_page_change.clone()}/>}
                        }).collect::<Html>()
                    }
                </div>
            </div>
        </div>
    }
}
