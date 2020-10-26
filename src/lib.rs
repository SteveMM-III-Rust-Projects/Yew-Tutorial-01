#![recursion_limit = "1024"]

mod app;
mod js_funcs;

use wasm_bindgen::prelude::*;
use app::App;


#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<App>();

    return Ok( () );
}
