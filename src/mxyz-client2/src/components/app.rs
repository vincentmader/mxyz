use crate::components::molecules::navbar_top::NavbarTop;
use crate::components::organisms::index::Index;
use crate::components::organisms::simulation::Simulation;
use mxyz_engine::config::simulation_variant::SimulationVariant;
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("../../../mxyz-server/static/css/base.css");

#[derive(PartialEq, yew::Properties, std::default::Default)]
pub struct Props {
    current_page: AppPage,
}

#[function_component(App)]
pub fn fn_name(props: &Props) -> Html {
    let style = stylist::Style::new(STYLE_FILE).unwrap();

    let page = match &props.current_page {
        AppPage::Index => html! {<Index />},
        AppPage::Simulation(_simulation_variant) => html! {<Simulation />},
        // _ => html! {<Simulation />},
    };
    html! {
        <div class={style}>
            <NavbarTop />
            <div class="page_content">
                {page}
            </div>
        </div>
    }
}

#[derive(std::default::Default, PartialEq)]
pub enum AppPage {
    #[default]
    Index,
    Simulation(SimulationVariant),
}
