use crate::components::molecules::navbar_top::NavbarTop;
use crate::components::organisms::index::Index;
use crate::components::organisms::simulation::Simulation;
use mxyz_client::client::EngineClient;
use mxyz_engine::config::engine_runner_variant::EngineRunnerVariant;
use mxyz_engine::config::simulation_variant::SimulationVariant;
use std::ops::Deref;
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("../../../mxyz-server/static/css/base.css");

#[derive(Clone, std::default::Default)]
pub struct AppState {
    current_page: AppPage,
}

#[function_component(App)]
pub fn fn_name() -> Html {
    let style = stylist::Style::new(STYLE_FILE).unwrap();

    let engine_runner_variant = EngineRunnerVariant::ClientWASM;
    // let a: Box<dyn EngineClient> = Box::from(&engine_runner_variant);
    let mut engine_runner: Box<dyn EngineClient> = (&engine_runner_variant).into();
    // engine_runner.init("nbody-gravity", "3body-moon");

    let app_state = use_state(AppState::default);

    let cloned_state = app_state.clone();
    let page_changed = Callback::from(move |page: AppPage| {
        cloned_state.set(AppState {
            current_page: page,
            ..cloned_state.deref().clone()
        });
    });

    let cloned_state = app_state.clone();
    let on_page_change = Callback::from(move |page: AppPage| {
        gloo::console::log!(format!("{:?}", page));
        cloned_state.set(AppState {
            current_page: page,
            ..cloned_state.deref().clone()
        });
        //
    });

    let page = match &app_state.current_page {
        AppPage::Index => html! {<Index on_page_change={on_page_change} />},
        AppPage::Simulation(_simulation_variant) => html! {<Simulation />},
        // _ => html! {<Simulation />},
    };
    html! {
        <div class={style}>
            <NavbarTop />
            {page}
        </div>
    }
}

#[derive(std::default::Default, PartialEq, Clone, Debug)]
pub enum AppPage {
    #[default]
    Index,
    Simulation(SimulationVariant),
}
