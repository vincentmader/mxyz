use super::navgrid_thumbnail::NavgridThumbnail;
use crate::components::app::AppPage;
use mxyz_engine::config::simulation_variant::SimulationVariant;
use yew::prelude::*;

const STYLE_FILE: &str =
    include_str!("../../../../../mxyz-server/static/css/index/navgrid_cell.css");

#[derive(Properties, PartialEq)]
pub struct Props {
    pub simulation_variant: SimulationVariant,
    pub on_page_change: Callback<AppPage>,
}

#[function_component(NavgridCell)]
pub fn fn_name(props: &Props) -> Html {
    let style = stylist::Style::new(STYLE_FILE).unwrap();

    let simulation_variant = props.simulation_variant.clone();
    let on_page_change_1 = props.on_page_change.clone();
    let on_page_change_1 = Callback::from(move |_| {
        on_page_change_1.emit(AppPage::Simulation(simulation_variant.clone()));
    });
    let simulation_variant = props.simulation_variant.clone();
    let on_page_change_2 = props.on_page_change.clone();
    let on_page_change_2 = Callback::from(move |_| {
        on_page_change_2.emit(AppPage::Simulation(simulation_variant.clone()));
    });

    html! {
        <div class={style}>
            <div class="navgrid_cell">
                <NavgridThumbnail simulation_variant={props.simulation_variant.clone()} on_page_change={on_page_change_2} />
                <span class="navgrid_cell_title" onclick={on_page_change_1}>
                    {props.simulation_variant.into_short_description_string()}
                </span>
            </div>
        </div>
    }
}
