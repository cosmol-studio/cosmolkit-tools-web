use cosmol_viewer_core::scene::Scene;
use dioxus::prelude::*;
pub const HOME_CANVAS_ID: &str = "home-canvas";

#[cfg(not(target_arch = "wasm32"))]
use crate::component::empty_viewer::Viewer;
#[cfg(target_arch = "wasm32")]
use crate::component::viewer::{Viewer, WasmLogger, get_viewer};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

const MOL_SVG: Asset = asset!("/assets/2-nitro-1H-pyrrole.svg");

fn build_molecule_scene() -> Scene {
    postcard::from_bytes(include_bytes!(concat!(
        env!("OUT_DIR"),
        "/home_scene.postcard"
    )))
    .expect("precomputed home scene should deserialize")
}

#[component]
pub fn Home() -> Element {
    let mut viewer: Signal<Option<Viewer>> = use_signal(|| None);
    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        let scene = build_molecule_scene();
        spawn_local(async move {
            match get_viewer(scene, HOME_CANVAS_ID).await {
                Ok(viewer_) => {
                    viewer.set(Some(viewer_));
                }
                Err(err) => {
                    // use crate::views::docking_renderer::WasmLogger;
                    use cosmol_viewer_core::Logger;

                    WasmLogger.log(format!("Failed to get docking viewer: {:?}", err));
                    println!("Failed to get docking viewer: {:?}", err)
                }
            }
        });
    });
    rsx! {
        main {
            class: "h-[1000px] uu-backdrop m-0 pt-[74px]",
            main{
                class: "max-w-6xl mx-auto py-5",
                div{
                    class: "flex max-w-5xl mx-auto mb-5 overflow-visible relative",
                    div {
                        class: "w-[50%] h-[300px] flex items-center justify-center flex-col z-10",
                        h1 {
                            class: "text-3xl font-bold leading-relaxed mb-4",
                            span {
                                class: "block text-white",
                                "Browser-based cheminformatics & structural biology tools,"
                            }
                            span {
                                class: "block text-[#3082FF]",
                                "powered by WebAssembly."
                            }
                        }
                        span {
                            class: "text-[white] leading-relaxed opacity-75 mb-4",
                            "Fast, private, and offline-capable tools for molecules and macromolecules.
                            All in your browser. No installation required."
                        }
                        div{
                            class: "flex gap-4 justify-left w-full",
                            button {
                                class: "bg-[#3082FF] text-white px-4 py-2 rounded cursor-pointer",
                                "Explore Tools"
                            }
                            Link {
                                class: "",
                                to: "https://kit.cosmol.org",
                                new_tab: true,
                                button {
                                    class: "text-white px-4 py-2 rounded cursor-pointer border border-white",
                                    "View Docs"
                                }
                            }
                        }
                    }
                    div {
                        class: "w-[50%] h-[300px] flex items-center justify-center flex-col absolute right-0 overflow-visible",
                        canvas {
                            class: "h-[140%] w-[140%]",
                            id: HOME_CANVAS_ID
                        }
                    }
                }
                div {
                    class: "w-full flex flex-col h-[500px]",
                    h2{
                        class: "text-3xl font-bold mb-4 text-xl text-white",
                        "Featured Tools"
                    }
                    div {
                        class: "w-full flex gap-4",
                        div {
                            class: "w-[300px] bg-[#ffffff04] rounded-xl flex flex-col px-[10px] py-[10px] border border-[#ffffff14]",
                            div{
                                class: "w-full h-100px flex space-between mb-2",
                                img {
                                    class: "h-[80px] w-[80px] m-[10px]",
                                    src: MOL_SVG,
                                }
                                div {
                                    class: "h-[100px] w-[180px] pl-[10px] justify-center flex flex-col",
                                    span {
                                        class: "block font-bold mb-2 text-white",
                                        "SMILES -> SVG"
                                    }
                                    span {
                                        class: "block leading-tight opacity-75 text-sm text-white",
                                        "Render SMILES string as scalable vector graphics."
                                    }
                                }
                            }
                            button {
                                class: "w-full text-white py-1.5 rounded-lg border border-[#3082FF80] border-[1.5px] cursor-pointer",
                                "Open"
                            }
                        }
                    }
                }
            }
        }
    }
}
