// use crate::components::molecules::login_form::LoginForm;
use crate::components::molecules::navgrid::Navgrid;
use yew::prelude::*;

#[function_component(Index)]
pub fn fn_name() -> Html {
    html! {
        <div>
            <Navgrid />
            // <LoginForm />
        </div>
    }
}

#[derive(PartialEq, yew::Properties, std::default::Default)]
pub struct Props {}
