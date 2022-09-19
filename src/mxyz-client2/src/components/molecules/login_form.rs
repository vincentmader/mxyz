use crate::components::atoms::button::Button;
use crate::components::atoms::text_input::TextInput;
use std::ops::Deref;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<Data>,
}

#[derive(Default, Clone)]
pub struct Data {
    pub username: String,
    pub password: String,
    pub count: u32,
}

#[function_component(LoginForm)]
pub fn login_form(props: &Props) -> Html {
    let state = use_state(|| Data::default());

    let cloned_state = state.clone();
    let username_changed = Callback::from(move |username: String| {
        cloned_state.set(Data {
            username,
            ..cloned_state.deref().clone()
        });
    });

    let cloned_state = state.clone();
    let password_changed = Callback::from(move |password: String| {
        cloned_state.set(Data {
            password,
            ..cloned_state.deref().clone()
        });
    });

    let cloned_state = state.clone();
    let submit_onclick = Callback::from(move |_| {
        let mut data = cloned_state.deref().clone();
        data.count += 1;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let form_onsubmit = props.onsubmit.clone();
    let form_onsubmit = Callback::from(move |event: FocusEvent| {
        event.prevent_default();
        let data = cloned_state.deref().clone();
        form_onsubmit.emit(data)
    });

    html! {
        <form onsubmit={form_onsubmit}>
            <TextInput label="username" onchange={username_changed}/>
            <TextInput label="password" onchange={password_changed}/>
            <Button label="Submit" onclick={submit_onclick}/>
            <p>{"Username: "}{&state.username}</p>
            <p>{"Password: "}{&state.password}</p>
            <p>{"Button clicked: "}{state.count}</p>
        </form>
    }
}
