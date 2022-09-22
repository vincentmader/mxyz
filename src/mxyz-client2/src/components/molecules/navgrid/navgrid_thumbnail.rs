use crate::components::app::AppPage;
use mxyz_engine::config::simulation_variant::SimulationVariant;
use yew::prelude::*;

const STYLE_FILE: &str =
    include_str!("../../../../../mxyz-server/static/css/index/navgrid_thumbnail.css",);

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub simulation_variant: SimulationVariant,
}

#[function_component(NavgridThumbnail)]
pub fn fn_name(props: &Props) -> Html {
    let style = stylist::Style::new(STYLE_FILE).unwrap();

    let simulation_variant = props.simulation_variant.clone();
    let path_to_thumbnail = simulation_variant.into_thumbnail_filename();

    let simulation_variant = props.simulation_variant.clone();
    let alt = simulation_variant.into_short_description_string();

    // let simulation_variant = props.simulation_variant.clone();
    // let foo = String::from(&props.simulation_variant.clone());
    let foo: String = (&props.simulation_variant.clone()).into();
    let foo = format!("/simulation/{}", foo);
    html! {
        <a class={style} href={foo}>
            <img class="navgrid_thumbnail active" src={path_to_thumbnail} alt={alt} />
        </a>
    }
}
