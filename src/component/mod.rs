pub mod navbar;
pub use navbar::Navbar;

pub mod home;
pub use home::Home;

pub mod icon;
pub use icon::MdiIcon;

#[cfg(not(target_arch = "wasm32"))]
mod empty_viewer;

#[cfg(target_arch = "wasm32")]
mod viewer;
