use cosmol_viewer_core::{App, Logger, scene::Scene};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsValue;

#[derive(Copy, Clone)]
pub struct RustLogger;

impl Logger for RustLogger {
    fn log(&self, message: impl std::fmt::Display) {
        println!("{message}");
    }

    fn warn(&self, message: impl std::fmt::Display) {
        println!("warn: {message}");
    }

    fn error(&self, message: impl std::fmt::Display) {
        println!("error: {message}");
    }
}

pub struct Viewer {
    pub app: Arc<Mutex<Option<App<RustLogger>>>>,
}

pub async fn get_viewer(scene: Scene) -> Result<Viewer, JsValue> {
    Err(JsValue::from_str(""))
}
