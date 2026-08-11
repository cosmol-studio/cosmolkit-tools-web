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

#[component]
fn App() -> Element {
    rsx! {
        ToastProvider {
            document::Link { rel: "icon", href: FAVICON }
            document::Link { rel: "stylesheet", href: MAIN_CSS }
            document::Link { rel: "stylesheet", href: TAILWIND_CSS }
            Router::<Route> {}
        }
    }
}
