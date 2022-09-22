use yew::prelude::*;

#[derive(PartialEq, yew::Properties, Default, Clone)]
pub struct Props {
    pub simulation_variant: Option<String>,
}

#[function_component(NavbarTop)]
pub fn fn_name(props: &Props) -> Html {
    let simulation_variant = match props.simulation_variant.clone() {
        Some(e) => e,
        _ => String::new(),
    };
    html! {
        <div class="page_header">
            <span class="home-button-container">
                <a href="/" class="home-button">{"mader.xyz"}</a>
            </span>
            <span class="">
                {simulation_variant}
            </span>
        </div>
    }
}
