#![forbid(unsafe_code)]
pub use crate::app::App;
#[cfg(target_arch = "wasm32")]
use {
    js_sys::Function,
    wasm_bindgen::prelude::{wasm_bindgen, JsValue},
};

mod app;
mod as_values;
mod csv_io;
#[cfg(not(target_arch = "wasm32"))]
pub mod icon;

/// WebAssembly entry point.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(id: &str, save_fn: &Function, load_fn: &Function) -> Result<(), JsValue> {
    let app = Box::new(App::with_hook(save_fn.clone(), load_fn.clone()));
    eframe::start_web(id, app)
}
