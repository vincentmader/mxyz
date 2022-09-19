use crate::components::app::AppPage;
// use crate::components::molecules::login_form::LoginForm;
use crate::components::molecules::navgrid::Navgrid;
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("../../../../../mxyz-server/static/css/index/base.css");

#[derive(PartialEq, yew::Properties)]
pub struct Props {
    pub on_page_change: Callback<AppPage>,
}

#[function_component(Index)]
pub fn fn_name(props: &Props) -> Html {
    let on_page_change = props.on_page_change.clone();

    let style = stylist::Style::new(STYLE_FILE).unwrap();

    // let custom_form_submit =
    //     Callback::from(|data: crate::components::molecules::login_form::Data| {
    //         gloo::console::log!("username is", data.username);
    //         gloo::console::log!("favorite language is", data.password);
    //     });

    html! {
        <div class={style}>
            // <LoginForm onsubmit={custom_form_submit} />
            <div class="page_content">
                <Navgrid on_page_change={on_page_change} />
            </div>
        </div>
    }
}
