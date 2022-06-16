/// Log to Browser Console
pub fn log(message: &str) {
    let array = js_sys::Array::new();
    array.push(&message.into());
    web_sys::console::log(&array);
}
