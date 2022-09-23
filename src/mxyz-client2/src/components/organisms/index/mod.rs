// use crate::components::app::AppPage;
use crate::components::molecules::navbar_top::NavbarTop;
// use crate::components::molecules::login_form::LoginForm;
use crate::components::molecules::navgrid::Navgrid;
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("../../../../../mxyz-server/static/css/index/base.css");

#[derive(PartialEq, yew::Properties, Clone)]
pub struct Props {}

#[function_component(Index)]
pub fn fn_name(props: &Props) -> Html {
    let style = stylist::Style::new(STYLE_FILE).unwrap();

    // let custom_form_submit =
    //     Callback::from(|data: crate::components::molecules::login_form::Data| {
    //         gloo::console::log!("username is", data.username);
    //         gloo::console::log!("favorite language is", data.password);
    //     });

    html! {
        <div class={style}>
            // <LoginForm onsubmit={custom_form_submit} />
            <NavbarTop />
            <div class="page_content">
                <Navgrid  />
            </div>
        </div>
    }
}
