pub mod app;
pub mod components;
pub mod consts;
pub mod error_template;
pub mod auth;
pub mod state;
pub mod canister;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod utils;
#[cfg(feature = "ssr")]
pub mod init;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
