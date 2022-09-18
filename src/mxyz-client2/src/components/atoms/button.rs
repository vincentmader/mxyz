use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub onclick: Callback<()>,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let onclick = props.onclick.clone();
    let button_onclick = Callback::from(move |_| {
        onclick.emit(());
    });
    html! {
        <div>
            <button onclick={button_onclick}>
                {props.label.clone()}
            </button>
        </div>
    }
}
