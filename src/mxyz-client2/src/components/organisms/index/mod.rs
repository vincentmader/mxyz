use crate::components::app::AppPage;
// use crate::components::molecules::login_form::LoginForm;
use crate::components::molecules::navgrid::Navgrid;
use yew::prelude::*;

#[derive(PartialEq, yew::Properties)]
pub struct Props {
    pub on_page_change: Callback<AppPage>,
}

#[function_component(Index)]
pub fn fn_name(props: &Props) -> Html {
    let on_page_change = props.on_page_change.clone();

    // let custom_form_submit =
    //     Callback::from(|data: crate::components::molecules::login_form::Data| {
    //         gloo::console::log!("username is", data.username);
    //         gloo::console::log!("favorite language is", data.password);
    //     });

    html! {
        <div>
            // <LoginForm onsubmit={custom_form_submit} />
            <Navgrid on_page_change={on_page_change} />
        </div>
    }
}
