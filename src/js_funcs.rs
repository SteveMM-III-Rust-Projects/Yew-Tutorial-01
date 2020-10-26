use wasm_bindgen::prelude::*;

#[wasm_bindgen(
   inline_js = "export function refreshform(form) {
      document.getElementById(form).reset();
   }"
)]
extern "C" {
   pub fn refreshform( form: &str );
}
