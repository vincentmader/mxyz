use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub label: String,
    pub onchange: Callback<String>,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let handle_onchange = props.onchange.clone();
    let onchange: Callback<Event> = Callback::from(move |event: Event| {
        let value: String = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        handle_onchange.emit(value);
    });
    html! {
        <div>
            <input
                type="text"
                name={props.label.clone()}
                placeholder={props.label.clone()}
                onchange={onchange}
            />
        </div>
    }
}
