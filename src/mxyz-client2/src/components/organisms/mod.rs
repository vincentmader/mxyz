pub mod index;
pub mod simulation;
use stylist::css;
use yew::prelude::*;

pub enum Msg {
    AddOne,
}

pub struct App {
    value: i64,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let text = String::from("Hello");
        gloo::console::log!("{:?}", &text);
        gloo::console::log!(serde_json::to_string_pretty(&text).unwrap());
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        let opt = Some("heyo");
        let arr = vec![1, 2, 3, 4, 5];
        // =============================================================================
        use stylist::style;
        // use stylist::yew::styled_component;
        let stylesheet = style!(
            r#"
            h1 {
                color: orange;
            }
            p {
                color: black;
            }
        "#
        )
        .unwrap();
        // =============================================================================
        use web_sys::{console, HtmlElement};
        // use yew::{html, Callback, TargetCast};

        let onmousemove = Callback::from(|e: MouseEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlElement>() {
                let rect = target.get_bounding_client_rect();
                let x = (e.client_x() as f64) - rect.left();
                let y = (e.client_y() as f64) - rect.top();
                console::log_1(&format!("Left? : {} ; Top? : {}", x, y).into());
            }
        });
        let stylesheet2 = style!(
            r#"background-color:green; 
            padding: 100px;
            "#
        )
        .unwrap();
        // =============================================================================
        html! {
            <div>

                // if statements
                if 2 > 1 {
                    <p>{"2>1!"}</p>
                } else {
                    <p>{"2<1!"}</p>
                }
                // if-let statements
                if let Some(_opt) = opt {
                    <p>{"Some"}</p>
                } else{
                    <p>{"None"}</p>
                }

                // html elements from iterators
                {arr.iter().map(|i| html!{<p>{i}</p>}).collect::<Html>()}

                // inline css
                <div class={css!("color: pink;")}></div>
                // css from rust "object"
                <div class={stylesheet}>
                    <h1>{"Hello world!"}</h1>
                    <p>{"more text..."}</p>
                </div>
                // css from included css file (?) TODO implement
                <div id="navgrid">
                    <div class="navgrid_section">
                        {"a"}
                        <div class="navgrid_cell">
                            {"a"}
                        </div>
                    </div>
                </div>

                // mouse events
                <div id="mousemoveme" class={stylesheet2} {onmousemove}>{"mouse"}</div>
                // button events
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

// =============================================================================

pub struct ComponentV1 {}
impl Component for ComponentV1 {
    type Message = usize;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {< >< />}
    }
}

// =============================================================================

pub enum IndexMessage {}

pub struct Index {}
impl Component for Index {
    type Message = IndexMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {< >< />}
    }
}

// =============================================================================
use web_sys::HtmlInputElement;
use yew::{html, Component, Context, Html, NodeRef};

#[derive(Properties, PartialEq, Clone)]
pub struct Props {}

pub enum Msg2 {
    SetInputEnabled(bool),
}

pub struct MyComponent {
    node_ref: NodeRef,
    input_enabled: bool,
}

impl Component for MyComponent {
    type Message = Msg2;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input_enabled: false,
            node_ref: NodeRef::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <input ref={self.node_ref.clone()} type="text" />
        }
    }

    // doesn't need to be implemented, will be called on default & do nothing
    // - called after render, before page refresh
    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(input) = self.node_ref.cast::<HtmlInputElement>() {
                input.focus();
            }
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg2::SetInputEnabled(enabled) => {
                if self.input_enabled != enabled {
                    self.input_enabled = enabled;
                    true // Re-render
                } else {
                    false
                }
            }
        }
    }
}
