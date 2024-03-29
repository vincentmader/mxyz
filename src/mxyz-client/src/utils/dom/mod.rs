use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// Window, Document, Body
// ============================================================================

/// Gets Window from DOM
pub fn window() -> web_sys::Window {
    web_sys::window().expect("ERROR: no global `window` exists")
}

/// Gets Document from DOM
pub fn document() -> web_sys::Document {
    window().document().expect("ERROR: no document on window")
}

/// Gets Page Body from Document
pub fn body() -> web_sys::HtmlElement {
    document().body().expect("ERROR: no body on document")
}

// ANIMATION FRAME
// TODO move whole Animation Loop from client.rs to here
// ============================================================================

/// Requests Animation Frame
pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("ERROR: should register `requestAnimationFrame` OK");
}

// LOG & DEBUG
// ============================================================================

/// Enables Panic Hook to console
///
/// When the `console_error_panic_hook` feature is enabled,
/// we can call the `set_panic_hook` function at least
/// once during initialization, and then we will get better
/// error messages if our code ever panics (-> sent to console).
///
/// For more details see
///     https://github.com/rustwasm/console_error_panic_hook#readme
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Performs JS Alert
#[wasm_bindgen]
extern "C" {
    // #[wasm_bindgen(js_namespace = console)]
    fn alert(s: &str);
    fn warn(s: &str);
    // fn log(s: &str);  // not working
}

// /// Manually Logs String as Warning to Browser Console
// macro_rules! console_warn {
//     ($($t:tt)*) => (warn(&format_args!($($t)*).to_string()))
// }
// pub(crate) use console_warn;

/// Manually Logs String to Browser Console
macro_rules! console_log {
    ($($t:tt)*) => {
        let msg = format_args!($($t)*).to_string();
        let array = js_sys::Array::new();
        array.push(&msg.into());
        web_sys::console::log(&array);
    };
}
pub(crate) use console_log;

// ============================================================================

//pub fn eventlistener_new_p_mousedown() {
//    let window = web_sys::window().expect("global window does not exists");
//    let document = window.document().expect("expecting a document on window");
//    let body = document
//        .body()
//        .expect("document expect to have have a body");

//    let paragraph = document
//        .create_element("p")
//        .unwrap()
//        .dyn_into::<web_sys::HtmlParagraphElement>()
//        .map_err(|_| ())
//        .unwrap();
//    paragraph.set_align("center");
//    paragraph.set_inner_html("<br />Click within this boundary to test the mousedown event. <br />Check the results in your web console.<br /><br />");
//    paragraph
//        .style()
//        .set_property("border", "solid")
//        .map_err(|_| ())
//        .unwrap();

//    let on_down = EventListener::new(&paragraph, "mousedown", move |_event| {
//        web_sys::console::log_1(&"Paragraph mousedown".into());
//    });
//    on_down.forget();
//    body.append_child(&paragraph).unwrap();
//}
//pub fn eventlistener_new_p_mousemove() {
//    let window = web_sys::window().expect("global window does not exists");
//    let document = window.document().expect("expecting a document on window");
//    let body = document
//        .body()
//        .expect("document expect to have have a body");

//    let paragraph = document
//        .create_element("p")
//        .unwrap()
//        .dyn_into::<web_sys::HtmlParagraphElement>()
//        .map_err(|_| ())
//        .unwrap();

//    paragraph.set_align("center");
//    paragraph.set_inner_html("<br />Move within this boundary to test the mousemove event. <br />Check the results in your web console.<br /><br />");

//    paragraph
//        .style()
//        .set_property("border", "solid")
//        .map_err(|_| ())
//        .unwrap();

//    let on_move = EventListener::new(&paragraph, "mousemove", move |_event| {
//        web_sys::console::log_1(&"Paragrapah mousemove".into());
//    });
//    on_move.forget();
//    body.append_child(&paragraph).unwrap();
//}

//pub fn eventlistener_new_p_mouseup() {
//    let window = web_sys::window().expect("global window does not exists");
//    let document = window.document().expect("expecting a document on window");
//    let body = document
//        .body()
//        .expect("document expect to have have a body");

//    let paragraph = document
//        .create_element("p")
//        .unwrap()
//        .dyn_into::<web_sys::HtmlParagraphElement>()
//        .map_err(|_| ())
//        .unwrap();

//    paragraph.set_align("center");
//    paragraph.set_inner_html("<br />Move within this boundary to test the mouseup event. <br />Check the results in your web console.<br /><br />");

//    paragraph
//        .style()
//        .set_property("border", "solid")
//        .map_err(|_| ())
//        .unwrap();

//    let on_up = EventListener::new(&paragraph, "mouseup", move |_event| {
//        web_sys::console::log_1(&"Paragrapah mouseup".into());
//    });
//    on_up.forget();
//    body.append_child(&paragraph).unwrap();
//}

// pub fn add_button_to_menu<F>(
//     text: &str,
//     // callback: &mut Box<F() -> ()>,
//     // callback: Fn() -> (),
//     // simulation: &'static mut Simulation,
//     // callback: &'static mut F,
//     callback: &'static mut Box<F>,
//     // callback: FnMut (),
//     // callback: fn(&mut Simulation),
// ) where
//     F: FnMut(),
// {
//     let document = document();
//     let section = document.get_element_by_id("button_menu-0").unwrap();

//     let button = document
//         .create_element("button")
//         .unwrap()
//         .dyn_into::<web_sys::HtmlButtonElement>()
//         .unwrap();
//     // button.set_id("button");
//     button.set_text_content(Some(text));

//     // let paragraph = document.create_element("p").unwrap()
//     // 	.dyn_into::<web_sys::HtmlParagraphElement>().unwrap();
//     // section.append_child(&paragraph).unwrap();

//     // use js_sys::Function;
//     // fn f() {}
//     // let f = || {}
//     // let f = Function::from(f);
//     // let optionn = Option::from(&f);
//     // button.set_onclick(optionn);

//     let on_click = web_sys::EventListener::new(&button, "click", move |_event| {
//         // paragraph.set_text_content(Some("Gloo: Hello World"));
//         // let a = (*callback);
//         callback();
//     });
//     on_click.forget();
//     section.append_child(&button).unwrap();
// }

//pub fn set_inner_html(id: &str, inner: &str) {
//    let document = document();
//    document
//        .get_element_by_id(id)
//        .unwrap()
//        .set_inner_html(inner);
//}
