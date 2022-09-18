use mxyz_engine::config::simulation_variant::SimulationVariant;
use yew::prelude::*;

const STYLE_FILE: &str =
    include_str!("../../../../../mxyz-server/static/css/index/navgrid_thumbnail.css",);

#[derive(Properties, PartialEq)]
pub struct Props {
    pub simulation_variant: SimulationVariant,
}

#[function_component(NavgridThumbnail)]
pub fn fn_name(props: &Props) -> Html {
    let style = stylist::Style::new(STYLE_FILE).unwrap();

    let path_to_thumbnail = props.simulation_variant.into_thumbnail_filename();
    let alt = props.simulation_variant.into_short_description_string();
    html! {
        <div class={style}>
            <img class="navgrid_thumbnail active" src={path_to_thumbnail} alt={alt} />
        </div>
    }
}
