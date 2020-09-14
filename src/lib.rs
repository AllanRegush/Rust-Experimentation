#![recursion_limit = "1024"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub mod app;
pub mod components;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<app::App>::new().mount_to_body();
}
