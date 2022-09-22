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
pub fn get_component() -> Html {
    let style = stylist::Style::new(STYLE_FILE).unwrap();

    let engine_runner_variant = EngineRunnerVariant::ClientWASM;
    // let a: Box<dyn EngineClient> = Box::from(&engine_runner_variant);
    let mut engine_runner: Box<dyn EngineClient> = (&engine_runner_variant).into();
    // engine_runner.init("nbody-gravity", "3body-moon");

    let app_state = use_state(AppState::default);

    let cloned_state = app_state.clone();
    let on_page_change = Callback::from(move |page: AppPage| {
        cloned_state.set(AppState {
            current_page: page,
            ..cloned_state.deref().clone()
        });
    });

    let page = match &app_state.current_page {
        AppPage::Index => html! {<Index on_page_change={on_page_change} />},
        AppPage::Simulation(_simulation_variant) => html! {<Simulation />},
        // _ => html! {<Simulation />},
    };
    html! {
        <div class={style}>
            <test::Model />
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

pub mod test {
    use super::producer::Producer;
    use super::subscriber::Subscriber;
    use yew::{html, Component, Context, Html};

    pub struct Model;

    impl Component for Model {
        type Message = ();
        type Properties = ();

        fn create(_ctx: &Context<Self>) -> Self {
            Self
        }

        fn view(&self, _ctx: &Context<Self>) -> Html {
            html! {
                <div style="text-align: center; padding-top:20px">
                    <Producer />
                    <Subscriber />
                </div>
            }
        }
    }
}

pub mod event_bus {
    use serde::{Deserialize, Serialize};
    use std::collections::HashSet;
    use yew_agent::{Agent, AgentLink, Context, HandlerId};

    #[derive(Serialize, Deserialize, Debug)]
    pub enum Request {
        EventBusMsg(String),
    }

    pub struct EventBus {
        link: AgentLink<EventBus>,
        subscribers: HashSet<HandlerId>,
    }

    impl Agent for EventBus {
        type Reach = Context<Self>;
        type Message = ();
        type Input = Request;
        type Output = String;

        fn create(link: AgentLink<Self>) -> Self {
            Self {
                link,
                subscribers: HashSet::new(),
            }
        }

        fn update(&mut self, _msg: Self::Message) {}

        fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
            match msg {
                Request::EventBusMsg(s) => {
                    for sub in self.subscribers.iter() {
                        self.link.respond(*sub, s.clone());
                    }
                }
            }
        }

        fn connected(&mut self, id: HandlerId) {
            self.subscribers.insert(id);
        }

        fn disconnected(&mut self, id: HandlerId) {
            self.subscribers.remove(&id);
        }
    }
}
pub mod subscriber {
    use super::event_bus::EventBus;
    use yew::{html, Component, Context, Html};
    use yew_agent::{Bridge, Bridged};

    pub enum Msg {
        NewMessage(String),
    }

    pub struct Subscriber {
        message: String,
        _producer: Box<dyn Bridge<EventBus>>,
    }

    impl Component for Subscriber {
        type Message = Msg;
        type Properties = ();

        fn create(ctx: &Context<Self>) -> Self {
            Self {
                message: "No message received yet.".to_owned(),
                _producer: EventBus::bridge(ctx.link().callback(Msg::NewMessage)),
            }
        }

        fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
            match msg {
                Msg::NewMessage(s) => {
                    self.message = s;
                    true
                }
            }
        }

        fn view(&self, _ctx: &Context<Self>) -> Html {
            html! {
                <h4 style="color: white;">{ &self.message }</h4>
            }
        }
    }
}
pub mod producer {
    use super::event_bus::{EventBus, Request};
    use yew::prelude::*;
    use yew_agent::{Dispatched, Dispatcher};

    pub enum Msg {
        Clicked,
    }

    pub struct Producer {
        event_bus: Dispatcher<EventBus>,
    }

    impl Component for Producer {
        type Message = Msg;
        type Properties = ();

        fn create(_ctx: &Context<Self>) -> Self {
            Self {
                event_bus: EventBus::dispatcher(),
            }
        }

        fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
            match msg {
                Msg::Clicked => {
                    self.event_bus.send(Request::EventBusMsg(
                        "Message received via EventBus".to_owned(),
                    ));
                    false
                }
            }
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            html! {
                <button onclick={ctx.link().callback(|_| Msg::Clicked)}>
                    { "Send Message via EventBus to other component" }
                </button>
            }
        }
    }
}
// pub mod yew_websocket_test {

//     use yew::format::Json;
//     use yew::prelude::*;
//     use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
//     use yew::services::ConsoleService;

//     struct Model {
//         console: ConsoleService,
//         ws: Option<WebSocketTask>,
//         wss: WebSocketService,
//         link: ComponentLink<Model>,
//         text: String,        // text in our input box
//         server_data: String, // data received from the server
//     }

//     enum Msg {
//         Connect,                         // connect to websocket server
//         Disconnected,                    // disconnected from server
//         Ignore,                          // ignore this message
//         TextInput(String),               // text was input in the input box
//         SendText,                        // send our text to server
//         Received(Result<String, Error>), // data received from server
//     }

//     impl Component for Model {
//         type Message = Msg;
//         type Properties = ();

//         fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
//             Model {
//                 console: ConsoleService::new(),
//                 ws: None,
//                 wss: WebSocketService::new(),
//                 link: link,
//                 text: String::new(),
//                 server_data: String::new(),
//             }
//         }

//         fn update(&mut self, msg: Self::Message) -> ShouldRender {
//             match msg {
//                 Msg::Connect => {
//                     self.console.log("Connecting");
//                     let cbout = self.link.send_back(|Json(data)| Msg::Received(data));
//                     let cbnot = self.link.send_back(|input| {
//                         ConsoleService::new().log(&format!("Notification: {:?}", input));
//                         match input {
//                             WebSocketStatus::Closed | WebSocketStatus::Error => Msg::Disconnected,
//                             _ => Msg::Ignore,
//                         }
//                     });
//                     if self.ws.is_none() {
//                         let task = self
//                             .wss
//                             .connect("ws://127.0.0.1:8080/ws/", cbout, cbnot.into());
//                         self.ws = Some(task);
//                     }
//                     true
//                 }
//                 Msg::Disconnected => {
//                     self.ws = None;
//                     true
//                 }
//                 Msg::Ignore => false,
//                 Msg::TextInput(e) => {
//                     self.text = e; // note input box value
//                     true
//                 }
//                 Msg::SendText => {
//                     match self.ws {
//                         Some(ref mut task) => {
//                             task.send(Json(&self.text));
//                             self.text = "".to_string();
//                             true // clear input box
//                         }
//                         None => false,
//                     }
//                 }
//                 Msg::Received(Ok(s)) => {
//                     self.server_data.push_str(&format!("{}\n", &s));
//                     true
//                 }
//                 Msg::Received(Err(s)) => {
//                     self.server_data.push_str(&format!(
//                         "Error when reading data from server: {}\n",
//                         &s.to_string()
//                     ));
//                     true
//                 }
//             }
//         }
//     }

//     impl Renderable<Model> for Model {
//         fn view(&self) -> Html<Self> {
//             html! {
//                 <>
//                     // connect button
//                     <p><button onclick=|_| Msg::Connect>{ "Connect" }</button></p><br/>
//                     // text showing whether we're connected or not
//                     <p>{ "Connected: " } { !self.ws.is_none() } </p><br/>
//                     // input box for sending text
//                     <p><input type="text" value=&self.text oninput=|e| Msg::TextInput(e.value) /></p><br/>
//                     // button for sending text
//                     <p><button onclick=|_| Msg::SendText>{ "Send" }</button></p><br/>
//                     // text area for showing data from the server
//                     <p><textarea value=&self.server_data></textarea></p><br/>
//                 </>
//             }
//         }
//     }
// }
