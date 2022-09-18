use yew::prelude::*;

#[derive(PartialEq, yew::Properties, std::default::Default)]
pub struct Props {}

#[function_component(NavbarTop)]
pub fn fn_name() -> Html {
    html! {
        <div class="page_header">
            <div class="home-button-container">
                <a href="/" class="home-button">{"mader.xyz"}</a>
            </div>
        </div>
    }
}
