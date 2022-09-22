use crate::components::app::AppPage;
use mxyz_engine::config::simulation_variant::SimulationVariant;
use yew::prelude::*;

const STYLE_FILE: &str =
    include_str!("../../../../../mxyz-server/static/css/index/navgrid_thumbnail.css",);

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub simulation_variant: SimulationVariant,
    pub on_page_change: Callback<AppPage>,
}

#[function_component(NavgridThumbnail)]
pub fn fn_name(props: &Props) -> Html {
    let style = stylist::Style::new(STYLE_FILE).unwrap();

    let path_to_thumbnail = props.simulation_variant.into_thumbnail_filename();
    let alt = props.simulation_variant.into_short_description_string();

    let simulation_variant = props.simulation_variant.clone();
    let on_page_change = props.on_page_change.clone();
    let on_page_change = Callback::from(move |_| {
        on_page_change.emit(AppPage::Simulation(simulation_variant.clone()));
    });
    html! {
        <div onclick={on_page_change} class={style}>
            <img class="navgrid_thumbnail active" src={path_to_thumbnail} alt={alt} />
        </div>
    }
}
