use dioxus::prelude::*;

use crate::{MOLECULE_SVG, SDF_SVG, component::Seo, route::Route};
pub const HOME_CANVAS_ID: &str = "home-canvas";

#[cfg(target_arch = "wasm32")]
use crate::component::{Viewer, WasmLogger, get_viewer};
#[cfg(target_arch = "wasm32")]
use cosmol_viewer_core::scene::Scene;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

#[cfg(target_arch = "wasm32")]
fn build_molecule_scene() -> Result<Scene, String> {
    postcard::from_bytes(include_bytes!(concat!(
        env!("OUT_DIR"),
        "/home_scene.postcard"
    )))
    .map_err(|error| format!("Could not load the precomputed home scene: {error}"))
}

#[component]
pub fn Home() -> Element {
    #[cfg(target_arch = "wasm32")]
    let mut viewer: Signal<Option<Viewer>> = use_signal(|| None);
    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        let scene = match build_molecule_scene() {
            Ok(scene) => scene,
            Err(error) => {
                use cosmol_viewer_core::Logger;
                WasmLogger.error(error);
                return;
            }
        };
        spawn_local(async move {
            match get_viewer(scene, HOME_CANVAS_ID).await {
                Ok(viewer_) => {
                    viewer.set(Some(viewer_));
                }
                Err(err) => {
                    use cosmol_viewer_core::Logger;

                    WasmLogger.log(format!("Failed to get docking viewer: {:?}", err));
                    println!("Failed to get docking viewer: {:?}", err)
                }
            }
        });
    });
    rsx! {
        Seo {
            title: "COSMolKit — Browser-Native Cheminformatics Powered by Rust",
            description: "Open-source browser-native cheminformatics tools powered by Rust, COSMolKit, and WebAssembly. Convert molecular formats, render SMILES, generate 3D conformers, calculate properties, and work with InChI locally.",
            canonical: "https://tools.cosmol.org/",
        }
        div {
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
                                "powered by Rust & WebAssembly."
                            }
                        }
                        span {
                            class: "text-[white] leading-relaxed opacity-75 mb-4",
                            "Built on the open-source COSMolKit Rust cheminformatics toolkit. Browser-native,
                            private, and offline-capable, with no installation required."
                        }
                        div{
                            class: "flex gap-4 justify-left w-full",
                            Link {
                                class: "bg-[#3082FF] text-white px-4 py-2 rounded cursor-pointer",
                                to: crate::route::Route::ToolDirectory {},
                                "Explore Tools"
                            }
                            Link {
                                class: "rounded border border-white px-4 py-2 text-white no-underline",
                                to: "https://kit.cosmol.org",
                                new_tab: true,
                                "View Docs"
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
                        class: "w-full flex flex-wrap gap-4",
                        div {
                            class: "w-[calc(25%-12px)] bg-[#ffffff04] rounded-xl flex flex-col px-[10px] py-[10px] border border-[#ffffff14] max-[1100px]:w-[calc(50%-8px)] max-[640px]:w-full",
                            div{
                                class: "w-full h-100px flex space-between mb-2",
                                div {
                                    class: "h-[60px] w-[60px] m-[10px] mt-[20px] border-[1.5px] border-[#1f93de90] rounded-xl",
                                    img {
                                        class: "w-[80%] h-[80%] m-[10%]",
                                        src: MOLECULE_SVG,
                                    }
                                }
                                div {
                                    class: "h-[100px] w-[180px] pl-[5px] justify-center flex flex-col",
                                    span {
                                        class: "block font-bold mb-2 text-white text-base",
                                        "SMILES -> SVG"
                                    }
                                    span {
                                        class: "block leading-tight opacity-75 text-sm text-white",
                                        "Render SMILES string as scalable vector graphics."
                                    }
                                }
                            }
                            Link {
                                class: "w-full text-center text-white py-1.5 rounded-lg border border-[#1f93de50] border-[1.5px] cursor-pointer hover:bg-[#3082FF80]",
                                to: crate::route::Route::SmilesToSvg {},
                                "Open"
                            }
                        }
                        div {
                            class: "w-[calc(25%-12px)] bg-[#ffffff04] rounded-xl flex flex-col px-[10px] py-[10px] border border-[#ffffff14] max-[1100px]:w-[calc(50%-8px)] max-[640px]:w-full",
                            div{
                                class: "w-full h-100px flex space-between mb-2",
                                div {
                                    class: "h-[60px] w-[60px] m-[10px] mt-[20px] border-[1.5px] border-[#71a554] rounded-xl",
                                    img {
                                        class: "w-[80%] h-[80%] m-[10%]",
                                        src: SDF_SVG,
                                    }
                                }
                                div {
                                    class: "h-[100px] w-[180px] pl-[5px] justify-center flex flex-col",
                                    span {
                                        class: "block font-bold mb-2 text-white text-base",
                                        "Format converter"
                                    }
                                    span {
                                        class: "block leading-tight opacity-75 text-sm text-white",
                                        "Convert SDF, SMILES, MOL2, PDB, mmCIF, and XYZ."
                                    }
                                }
                            }
                            Link {
                                class: "w-full text-center text-white py-1.5 rounded-lg border border-[#71a55450] border-[1.5px] cursor-pointer hover:bg-[#71a55480]",
                                to: crate::route::Route::FormatConverter {},
                                "Open"
                            }
                        }
                        div {
                            class: "w-[calc(25%-12px)] bg-[#ffffff04] rounded-xl flex flex-col px-[10px] py-[10px] border border-[#ffffff14] max-[1100px]:w-[calc(50%-8px)] max-[640px]:w-full",
                            div{
                                class: "w-full h-100px flex space-between mb-2",
                                div {
                                    class: "h-[60px] w-[60px] m-[10px] mt-[20px] border-[1.5px] border-[#1f93de90] rounded-xl",
                                    img {
                                        class: "w-[80%] h-[80%] m-[10%]",
                                        src: MOLECULE_SVG,
                                    }
                                }
                                div {
                                    class: "h-[100px] w-[180px] pl-[5px] justify-center flex flex-col",
                                    span {
                                        class: "block font-bold mb-2 text-white text-base",
                                        "SMILES to 3D"
                                    }
                                    span {
                                        class: "block leading-tight opacity-75 text-sm text-white",
                                        "Generate deterministic 3D molecular coordinates."
                                    }
                                }
                            }
                            Link {
                                class: "w-full text-white text-center py-1.5 rounded-lg border border-[#1f93de50] border-[1.5px] cursor-pointer hover:bg-[#3082FF80]",
                                to: Route::ConformerGenerator {},
                                "Open"
                            }
                        }
                        div {
                            class: "w-[calc(25%-12px)] bg-[#ffffff04] rounded-xl flex flex-col px-[10px] py-[10px] border border-[#ffffff14] max-[1100px]:w-[calc(50%-8px)] max-[640px]:w-full",
                            div {
                                class: "w-full h-100px flex space-between mb-2",
                                div {
                                    class: "h-[60px] w-[60px] m-[10px] mt-[20px] border-[1.5px] border-[#55a6c8] rounded-xl",
                                    img {
                                        class: "w-[80%] h-[80%] m-[10%]",
                                        src: SDF_SVG,
                                    }
                                }
                                div {
                                    class: "h-[100px] w-[180px] pl-[5px] justify-center flex flex-col",
                                    span {
                                        class: "block font-bold mb-2 text-white text-base",
                                        "InChI converter"
                                    }
                                    span {
                                        class: "block leading-tight opacity-75 text-sm text-white",
                                        "Generate identifiers or recover molecular structures."
                                    }
                                }
                            }
                            Link {
                                class: "w-full text-white text-center py-1.5 rounded-lg border border-[#55a6c850] border-[1.5px] cursor-pointer hover:bg-[#55a6c880]",
                                to: Route::InchiTool {},
                                "Open"
                            }
                        }
                    }
                }
            }
        }
    }
}
