pub mod button;
pub mod main_title;
pub mod text_input;

// pub mod button {
//     use yew::{html, Component, Context, Html, Properties};

//     #[derive(PartialEq, Properties)]
//     pub struct Props {}

//     pub struct Button {
//         variant: ButtonVariant,
//     }
//     impl Component for Button {
//         type Message = ();
//         type Properties = Props;

//         fn create(_ctx: &Context<Self>) -> Self {
//             let variant = todo!();
//             Self { variant }
//         }

//         fn view(&self, _ctx: &Context<Self>) -> Html {
//             html! {
//                 <>
//                 </>
//             }
//         }

//         fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
//             true
//         }
//     }

//     pub enum ButtonVariant {
//         Generic,
//     }
// }

// =============================================================================

pub mod link {
    use yew::virtual_dom::AttrValue;
    use yew::{html, Context, Html};

    #[derive(PartialEq, yew::Properties, Clone)]
    pub struct LinkProps {
        href: AttrValue,
        text: AttrValue,
        #[prop_or_default]
        size: Option<u32>,
        #[prop_or(true)]
        active: bool,
    }

    pub struct Link {}
    impl yew::Component for Link {
        type Message = ();
        type Properties = LinkProps;

        fn create(_ctx: &Context<Self>) -> Self {
            Self {}
        }

        fn view(&self, _ctx: &Context<Self>) -> Html {
            html! {
                <>
                </>
            }
        }

        fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
            true
        }
    }
}
