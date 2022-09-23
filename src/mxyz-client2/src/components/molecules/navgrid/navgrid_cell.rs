use super::navgrid_thumbnail::NavgridThumbnail;
use crate::components::app::AppPage;
use mxyz_engine::config::simulation_variant::SimulationVariant;
use yew::prelude::*;

const STYLE_FILE: &str =
    include_str!("../../../../../mxyz-server/static/css/index/navgrid_cell.css");

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub simulation_variant: SimulationVariant,
}

#[function_component(NavgridCell)]
pub fn fn_name(props: &Props) -> Html {
    let style = stylist::Style::new(STYLE_FILE).unwrap();

    let foo: String = (&props.simulation_variant.clone()).into();
    let foo = format!("/simulation/{}", foo);
    html! {
        <div class={style}>
            <div class="navgrid_cell">
                <NavgridThumbnail simulation_variant={props.simulation_variant.clone()} />
                <a class="navgrid_cell_title" href={foo}>
                    {props.simulation_variant.into_short_description_string()}
                </a>
            </div>
        </div>
    }
}
