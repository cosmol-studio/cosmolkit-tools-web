use cosmol_viewer_core::eframe::{WebOptions, WebRunner};
use cosmol_viewer_core::scene::Scene;
use cosmol_viewer_core::{App, AppWrapper, Logger};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::HtmlCanvasElement;

#[derive(Clone, Copy)]
pub struct WasmLogger;

impl Logger for WasmLogger {
    fn log(&self, message: impl std::fmt::Display) {
        web_sys::console::log_1(&JsValue::from_str(&message.to_string()));
    }

    fn warn(&self, message: impl std::fmt::Display) {
        web_sys::console::warn_1(&JsValue::from_str(&message.to_string()));
    }

    fn error(&self, message: impl std::fmt::Display) {
        let msg = message.to_string();
        web_sys::console::error_1(&JsValue::from_str(&msg));
        if let Some(window) = web_sys::window() {
            window.alert_with_message(&msg).ok();
        }
    }
}

pub struct Viewer {
    _app: Arc<Mutex<Option<App<WasmLogger>>>>,
    pub _runner: WebRunner,
}

pub async fn init_render(scene: Scene, canvas: HtmlCanvasElement) -> Result<Viewer, JsValue> {
    let app = Arc::new(Mutex::new(None));
    let app_ = app.clone();
    let runner = WebRunner::new();

    let web_options = WebOptions {
        should_stop_propagation: Box::new(|event| {
            !matches!(
                event,
                cosmol_viewer_core::eframe::egui::Event::MouseWheel { .. }
            )
        }),
        should_prevent_default: Box::new(|event| {
            !matches!(
                event,
                cosmol_viewer_core::eframe::egui::Event::MouseWheel { .. }
            )
        }),
        ..WebOptions::default()
    };

    let _ = runner
        .start(
            canvas,
            web_options,
            Box::new(move |cc| {
                let mut guard = app_.lock().unwrap_or_else(|error| error.into_inner());
                *guard = Some(App::new(cc, &scene, WasmLogger));
                Ok(Box::new(AppWrapper(app_.clone())))
            }),
        )
        .await
        .map_err(|err| JsValue::from_str(&format!("embedded viewer start failed: {err:?}")));

    Ok(Viewer {
        _app: app,
        _runner: runner,
    })
}

#[cfg(target_arch = "wasm32")]
pub async fn get_viewer(scene: Scene, canvas_id: &str) -> Result<Viewer, JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("window is not available"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("document is not available"))?;
    let canvas = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| JsValue::from_str("docking-canvas is not found"))?
        .dyn_into::<HtmlCanvasElement>()?;

    init_render(scene, canvas).await
}
