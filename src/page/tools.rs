use dioxus::prelude::*;

const MOLECULE_SVG: Asset = asset!("/assets/benzene.svg");
const SDF_SVG: Asset = asset!("/assets/sdf.svg");

#[component]
pub fn ToolDirectory() -> Element {
    rsx! {
        document::Title { "Tools | COSMolkit Tools" }
        div { class: "min-h-screen uu-backdrop m-0 pt-[74px]",
            main { class: "mx-auto w-full max-w-6xl px-6 py-10 font-sans text-[#e8edf5] max-[640px]:px-3.5 max-[640px]:py-7",
                header { class: "flex items-end justify-between gap-8 border-b border-white/10 pb-7 max-[700px]:flex-col max-[700px]:items-start max-[700px]:gap-4",
                    div {
                        span { class: "text-xs font-bold text-[#4b96ff]", "TOOL DIRECTORY" }
                        h1 { class: "mt-2 mb-2 text-[32px] leading-tight font-bold text-white max-[640px]:text-[27px]", "Tools" }
                        p { class: "m-0 max-w-[650px] text-[15px] leading-6 text-[#9caabd]",
                            "Browser-native molecular utilities powered by the COSMolKit Rust core."
                        }
                    }
                    div { class: "flex shrink-0 items-center gap-3 text-xs",
                        span { class: "rounded-md border border-[#285b4d] bg-[#0d2923] px-2.5 py-1.5 font-semibold text-[#8ee0c4]", "4 available" }
                        span { class: "text-[#718299]", "5 tools" }
                    }
                }

                section { class: "mt-8 grid grid-cols-3 gap-4 max-[900px]:grid-cols-2 max-[640px]:grid-cols-1",
                    Link {
                        class: "group flex min-h-[292px] flex-col rounded-lg border border-[#28415f] bg-[#0b1727] p-5 no-underline shadow-[0_16px_40px_rgba(0,0,0,0.16)] transition-colors hover:border-[#438ee9] hover:bg-[#0d1b2d]",
                        to: crate::route::Route::SmilesToSvg {},
                        div { class: "flex items-start justify-between gap-4",
                            div { class: "grid h-16 w-16 shrink-0 place-items-center rounded-lg border border-[#1f93de90] bg-[#0f2033] p-2.5",
                                img { class: "h-full w-full object-contain", src: MOLECULE_SVG, alt: "Benzene structure" }
                            }
                            span { class: "rounded-[5px] border border-[#285b4d] bg-[#0d2923] px-2 py-1 text-[10px] font-bold text-[#8ee0c4]", "AVAILABLE" }
                        }
                        div { class: "mt-6",
                            span { class: "text-[10px] font-bold text-[#4b96ff]", "MOLECULE DEPICTION" }
                            h2 { class: "mt-2 mb-2 text-lg font-bold text-white", "SMILES to SVG" }
                            p { class: "m-0 text-[13px] leading-5 text-[#9caabd]",
                                "Render SMILES as scalable 2D molecular graphics and export production-ready SVG."
                            }
                        }
                        div { class: "mt-auto flex items-center justify-between border-t border-white/8 pt-4 text-xs font-semibold text-[#7ab5ff]",
                            span { "Open tool" }
                            span { class: "text-base transition-transform group-hover:translate-x-1", ">" }
                        }
                    }

                    Link {
                        class: "group flex min-h-[292px] flex-col rounded-lg border border-[#31503e] bg-[#0b1727] p-5 no-underline shadow-[0_16px_40px_rgba(0,0,0,0.16)] transition-colors hover:border-[#65a56f] hover:bg-[#0d1b2d]",
                        to: crate::route::Route::FormatConverter {},
                        div { class: "flex items-start justify-between gap-4",
                            div { class: "grid h-16 w-16 shrink-0 place-items-center rounded-lg border border-[#71a55480] bg-[#14231d] p-2.5",
                                img { class: "h-full w-full object-contain", src: SDF_SVG, alt: "Molecular file" }
                            }
                            span { class: "rounded-[5px] border border-[#285b4d] bg-[#0d2923] px-2 py-1 text-[10px] font-bold text-[#8ee0c4]", "AVAILABLE" }
                        }
                        div { class: "mt-6",
                            span { class: "text-[10px] font-bold text-[#86ad72]", "MOLECULAR I/O" }
                            h2 { class: "mt-2 mb-2 text-lg font-bold text-white", "Format converter" }
                            p { class: "m-0 text-[13px] leading-5 text-[#9caabd]",
                                "Read six common molecular formats and export SMILES, MOL, SDF, PDB, or SVG."
                            }
                        }
                        div { class: "mt-auto flex items-center justify-between border-t border-white/8 pt-4 text-xs font-semibold text-[#8cc596]",
                            span { "Open tool" }
                            span { class: "text-base transition-transform group-hover:translate-x-1", ">" }
                        }
                    }

                    Link {
                        class: "group flex min-h-[292px] flex-col rounded-lg border border-[#4b4568] bg-[#0b1727] p-5 no-underline shadow-[0_16px_40px_rgba(0,0,0,0.16)] transition-colors hover:border-[#8d7bd0] hover:bg-[#0d1b2d]",
                        to: crate::route::Route::ConformerGenerator {},
                        div { class: "flex items-start justify-between gap-4",
                            div { class: "grid h-16 w-16 shrink-0 place-items-center rounded-lg border border-[#8d7bd080] bg-[#1c1930] p-2.5",
                                img { class: "h-full w-full object-contain", src: MOLECULE_SVG, alt: "Molecular structure" }
                            }
                            span { class: "rounded-[5px] border border-[#285b4d] bg-[#0d2923] px-2 py-1 text-[10px] font-bold text-[#8ee0c4]", "AVAILABLE" }
                        }
                        div { class: "mt-6",
                            span { class: "text-[10px] font-bold text-[#ab9de0]", "3D COORDINATES" }
                            h2 { class: "mt-2 mb-2 text-lg font-bold text-white", "Conformer generator" }
                            p { class: "m-0 text-[13px] leading-5 text-[#9caabd]",
                                "Generate deterministic 3D coordinates with ETKDG and export SDF or PDB."
                            }
                        }
                        div { class: "mt-auto flex items-center justify-between border-t border-white/8 pt-4 text-xs font-semibold text-[#b7a9eb]",
                            span { "Open tool" }
                            span { class: "text-base transition-transform group-hover:translate-x-1", ">" }
                        }
                    }

                    Link {
                        class: "group flex min-h-[292px] flex-col rounded-lg border border-[#315266] bg-[#0b1727] p-5 no-underline shadow-[0_16px_40px_rgba(0,0,0,0.16)] transition-colors hover:border-[#55a6c8] hover:bg-[#0d1b2d]",
                        to: crate::route::Route::InchiTool {},
                        div { class: "flex items-start justify-between gap-4",
                            div { class: "grid h-16 w-16 shrink-0 place-items-center rounded-lg border border-[#55a6c880] bg-[#102431] p-2.5",
                                img { class: "h-full w-full object-contain", src: SDF_SVG, alt: "Molecular identifier" }
                            }
                            span { class: "rounded-[5px] border border-[#285b4d] bg-[#0d2923] px-2 py-1 text-[10px] font-bold text-[#8ee0c4]", "AVAILABLE" }
                        }
                        div { class: "mt-6",
                            span { class: "text-[10px] font-bold text-[#72bddb]", "CHEMICAL IDENTIFIERS" }
                            h2 { class: "mt-2 mb-2 text-lg font-bold text-white", "InChI workspace" }
                            p { class: "m-0 text-[13px] leading-5 text-[#9caabd]",
                                "Generate InChI and InChIKey values or recover structures from standard InChI."
                            }
                        }
                        div { class: "mt-auto flex items-center justify-between border-t border-white/8 pt-4 text-xs font-semibold text-[#86c8e2]",
                            span { "Open tool" }
                            span { class: "text-base transition-transform group-hover:translate-x-1", ">" }
                        }
                    }

                    Link {
                        class: "group flex min-h-[292px] flex-col rounded-lg border border-[#4b3f34] bg-[#0a1524]/80 p-5 no-underline transition-colors hover:border-[#806b45] hover:bg-[#0d1b2d]",
                        to: crate::route::Route::CheckPains {},
                        div { class: "flex items-start justify-between gap-4",
                            div { class: "grid h-16 w-16 shrink-0 place-items-center rounded-lg border border-[#9b6b8080] bg-[#241823] p-2.5",
                                img { class: "h-full w-full object-contain opacity-80", src: MOLECULE_SVG, alt: "Chemical structure" }
                            }
                            span { class: "rounded-[5px] border border-[#5d5132] bg-[#282315] px-2 py-1 text-[10px] font-bold text-[#d9bd72]", "CORE PENDING" }
                        }
                        div { class: "mt-6",
                            span { class: "text-[10px] font-bold text-[#c38da5]", "COMPOUND FILTERS" }
                            h2 { class: "mt-2 mb-2 text-lg font-bold text-[#d8e0ea]", "Check PAINS" }
                            p { class: "m-0 text-[13px] leading-5 text-[#8290a2]",
                                "Screen molecular structures for pan-assay interference compound patterns."
                            }
                        }
                        div { class: "mt-auto flex items-center justify-between border-t border-white/8 pt-4 text-xs font-semibold text-[#b5a26d]",
                            span { "View status" }
                            span { class: "text-base transition-transform group-hover:translate-x-1", ">" }
                        }
                    }
                }

                footer { class: "mt-8 flex items-center gap-2 border-t border-white/8 pt-5 text-xs text-[#718299]",
                    span { class: "h-2 w-2 rounded-full bg-[#38c793]" }
                    "Available tools execute locally in your browser through Rust and WebAssembly."
                }
            }
        }
    }
}
