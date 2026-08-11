pub(crate) mod icon;
pub(crate) mod navbar;
pub(crate) mod seo;
pub(crate) mod toast;

pub(crate) use icon::MdiIcon;
pub(crate) use navbar::Navbar;
pub(crate) use seo::Seo;
pub(crate) use toast::{ToastManager, ToastProvider};

#[cfg(target_arch = "wasm32")]
mod viewer;
#[cfg(target_arch = "wasm32")]
pub(crate) use viewer::{Viewer, WasmLogger, get_viewer};
