pub mod app;
pub mod components;
pub mod models;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("Couldn't initialize logging");
    leptos::mount::hydrate_body(App);
}
