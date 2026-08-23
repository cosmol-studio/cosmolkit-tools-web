#![cfg_attr(feature = "wasm-split", allow(non_snake_case))]

mod component;
mod page;
mod route;
use component::ToastProvider;
use dioxus::prelude::*;
use route::Route;

const FAVICON: Asset = asset!("/assets/logo.svg");
const MAIN_CSS: Asset = asset!("/assets/main.css");
// const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
pub(crate) const MOLECULE_SVG: Asset = asset!("/assets/benzene.svg");
pub(crate) const SDF_SVG: Asset = asset!("/assets/sdf.svg");

#[cfg(all(feature = "ssg", not(target_arch = "wasm32")))]
fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(server_only! {
            dioxus::server::ServeConfig::builder()
                .incremental(
                    dioxus::server::IncrementalRendererConfig::new()
                        .static_dir(
                            std::env::current_exe()
                                .expect("current executable path")
                                .parent()
                                .expect("server executable directory")
                                .join("public"),
                        )
                        .clear_cache(false),
                )
                .enable_out_of_order_streaming()
        })
        .launch(App);
}

#[cfg(any(not(feature = "ssg"), target_arch = "wasm32"))]
fn main() {
    #[cfg(all(target_arch = "wasm32", feature = "wasm-split"))]
    if let Some(root) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.get_element_by_id("main"))
    {
        // Dioxus 0.7.10 route splitting cannot hydrate SSG output yet. Keep the
        // prerendered document for crawlers, then mount the split client cleanly.
        root.set_inner_html("");
    }

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        span { id: "wasm-ready", hidden: true }
        InitialLoader {}
        ToastProvider {
            document::Link { rel: "icon", href: FAVICON }
            document::Link { rel: "stylesheet", href: MAIN_CSS }
            document::Link { rel: "stylesheet", href: TAILWIND_CSS }
            Router::<Route> {}
        }
    }
}

#[component]
fn InitialLoader() -> Element {
    #[allow(unused_mut)]
    let mut visible = use_signal(|| cfg!(target_arch = "wasm32") || cfg!(feature = "ssg"));

    #[cfg(target_arch = "wasm32")]
    use_effect(move || visible.set(false));

    if !visible() {
        return rsx! {};
    }

    rsx! {
        div {
            id: "initial-loader",
            role: "status",
            aria_live: "polite",
            aria_label: "Loading COSMolKit Tools",
            div { class: "initial-loader-content",
                p { class: "initial-loader-brand", span { "COSMolKit" } " Tools" }
                p { class: "initial-loader-status", "Loading browser workspace" }
                span { class: "initial-loader-progress", aria_hidden: "true" }
            }
        }
    }
}
