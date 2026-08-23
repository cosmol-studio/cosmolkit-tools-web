use dioxus::prelude::*;

use crate::component::{MoleculeCardIcon, SdfCardIcon, Seo};

#[component]
pub fn ToolDirectory() -> Element {
    rsx! {
        Seo {
            title: "Browser-Based Cheminformatics Tools Powered by Rust — COSMolKit",
            description: "Free browser-based cheminformatics tools powered by Rust and COSMolKit for molecular properties, SMILES canonicalization, format conversion, SVG depiction, ETKDG conformers, and InChI workflows.",
            canonical: "https://tools.cosmol.org/tools",
        }
        div { class: "min-h-screen uu-backdrop m-0 pt-[74px]",
            main { class: "mx-auto w-full max-w-6xl px-6 py-10 font-sans text-[#e8edf5] max-[640px]:px-3.5 max-[640px]:py-7",
                header { class: "flex items-end justify-between gap-8 border-b border-white/10 pb-7 max-[700px]:flex-col max-[700px]:items-start max-[700px]:gap-4",
                    div {
                        span { class: "text-xs font-bold text-[#4b96ff]", "TOOL DIRECTORY" }
                        h1 { class: "mt-2 mb-2 text-[32px] leading-tight font-bold text-white max-[640px]:text-[27px]", "Free Browser-Based Cheminformatics Tools" }
                        p { class: "m-0 max-w-[650px] text-[15px] leading-6 text-[#9caabd]",
                            "Powered by Rust and the open-source COSMolKit cheminformatics library, these molecular tools run locally through WebAssembly."
                        }
                    }
                    div { class: "flex shrink-0 items-center gap-3 text-xs",
                        span { class: "rounded-md border border-[#285b4d] bg-[#0d2923] px-2.5 py-1.5 font-semibold text-[#8ee0c4]", "6 available" }
                        span { class: "text-[#718299]", "7 tools" }
                    }
                }

                section { class: "mt-8 grid grid-cols-3 gap-4 max-[900px]:grid-cols-2 max-[640px]:grid-cols-1",
                    Link {
                        class: "group flex min-h-[292px] flex-col rounded-lg border border-[#28415f] bg-[#0b1727] p-5 no-underline shadow-[0_16px_40px_rgba(0,0,0,0.16)] transition-colors hover:border-[#438ee9] hover:bg-[#0d1b2d]",
                        to: crate::route::Route::SmilesToSvg {},
                        div { class: "flex items-start justify-between gap-4",
                            div { class: "grid h-16 w-16 shrink-0 place-items-center rounded-lg border border-[#1f93de90] bg-[#0f2033] p-2.5",
                                MoleculeCardIcon { class: "h-full w-full", label: "Benzene structure" }
                            }
                            span { class: "rounded-[5px] border border-[#285b4d] bg-[#0d2923] px-2 py-1 text-[10px] font-bold text-[#8ee0c4]", "AVAILABLE" }
                        }
                        div { class: "mt-6",
                            span { class: "text-[10px] font-bold text-[#4b96ff]", "MOLECULE DEPICTION" }
                            h2 { class: "mt-2 mb-2 text-lg font-bold text-white", "SMILES to SVG" }
                            p { class: "m-0 text-[13px] leading-5 text-[#9caabd]",
                                "Render 2D molecular structures from SMILES and export scalable SVG files."
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
                                SdfCardIcon { class: "h-full w-full", label: "Molecular file" }
                            }
                            span { class: "rounded-[5px] border border-[#285b4d] bg-[#0d2923] px-2 py-1 text-[10px] font-bold text-[#8ee0c4]", "AVAILABLE" }
                        }
                        div { class: "mt-6",
                            span { class: "text-[10px] font-bold text-[#86ad72]", "MOLECULAR I/O" }
                            h2 { class: "mt-2 mb-2 text-lg font-bold text-white", "Molecular format converter" }
                            p { class: "m-0 text-[13px] leading-5 text-[#9caabd]",
                                "Convert SDF to SMILES, SMILES to SDF, MOL2 to PDB, mmCIF files, XYZ coordinates, and more."
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
                                MoleculeCardIcon { class: "h-full w-full", label: "Molecular structure" }
                            }
                            span { class: "rounded-[5px] border border-[#285b4d] bg-[#0d2923] px-2 py-1 text-[10px] font-bold text-[#8ee0c4]", "AVAILABLE" }
                        }
                        div { class: "mt-6",
                            span { class: "text-[10px] font-bold text-[#ab9de0]", "3D COORDINATES" }
                            h2 { class: "mt-2 mb-2 text-lg font-bold text-white", "SMILES to 3D conformer" }
                            p { class: "m-0 text-[13px] leading-5 text-[#9caabd]",
                                "Generate ETKDG 3D conformers from SMILES and export SDF or PDB structures."
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
                                SdfCardIcon { class: "h-full w-full", label: "Molecular identifier" }
                            }
                            span { class: "rounded-[5px] border border-[#285b4d] bg-[#0d2923] px-2 py-1 text-[10px] font-bold text-[#8ee0c4]", "AVAILABLE" }
                        }
                        div { class: "mt-6",
                            span { class: "text-[10px] font-bold text-[#72bddb]", "CHEMICAL IDENTIFIERS" }
                            h2 { class: "mt-2 mb-2 text-lg font-bold text-white", "InChI and InChIKey converter" }
                            p { class: "m-0 text-[13px] leading-5 text-[#9caabd]",
                                "Generate standard InChI and InChIKey values or parse InChI back to molecular structures."
                            }
                        }
                        div { class: "mt-auto flex items-center justify-between border-t border-white/8 pt-4 text-xs font-semibold text-[#86c8e2]",
                            span { "Open tool" }
                            span { class: "text-base transition-transform group-hover:translate-x-1", ">" }
                        }
                    }

                    Link {
                        class: "group flex min-h-[292px] flex-col rounded-lg border border-[#514a32] bg-[#0b1727] p-5 no-underline shadow-[0_16px_40px_rgba(0,0,0,0.16)] transition-colors hover:border-[#c4a84e] hover:bg-[#0d1b2d]",
                        to: crate::route::Route::MolecularProperties {},
                        div { class: "flex items-start justify-between gap-4",
                            div { class: "grid h-16 w-16 shrink-0 place-items-center rounded-lg border border-[#c4a84e80] bg-[#282416] p-2.5",
                                MoleculeCardIcon { class: "h-full w-full", label: "Molecule property calculation" }
                            }
                            span { class: "rounded-[5px] border border-[#285b4d] bg-[#0d2923] px-2 py-1 text-[10px] font-bold text-[#8ee0c4]", "AVAILABLE" }
                        }
                        div { class: "mt-6",
                            span { class: "text-[10px] font-bold text-[#d2b95e]", "MOLECULAR DESCRIPTORS" }
                            h2 { class: "mt-2 mb-2 text-lg font-bold text-white", "Molecular properties calculator" }
                            p { class: "m-0 text-[13px] leading-5 text-[#9caabd]",
                                "Calculate formula, molecular weight, exact mass, TPSA, logP, HBD, HBA, rotatable bonds, and charge."
                            }
                        }
                        div { class: "mt-auto flex items-center justify-between border-t border-white/8 pt-4 text-xs font-semibold text-[#d8c574]",
                            span { "Open tool" }
                            span { class: "text-base transition-transform group-hover:translate-x-1", ">" }
                        }
                    }

                    Link {
                        class: "group flex min-h-[292px] flex-col rounded-lg border border-[#3d4f55] bg-[#0b1727] p-5 no-underline shadow-[0_16px_40px_rgba(0,0,0,0.16)] transition-colors hover:border-[#6ba6b5] hover:bg-[#0d1b2d]",
                        to: crate::route::Route::SmilesCanonicalizer {},
                        div { class: "flex items-start justify-between gap-4",
                            div { class: "grid h-16 w-16 shrink-0 place-items-center rounded-lg border border-[#6ba6b580] bg-[#14252a] p-2.5",
                                SdfCardIcon { class: "h-full w-full", label: "Canonical SMILES output" }
                            }
                            span { class: "rounded-[5px] border border-[#285b4d] bg-[#0d2923] px-2 py-1 text-[10px] font-bold text-[#8ee0c4]", "AVAILABLE" }
                        }
                        div { class: "mt-6",
                            span { class: "text-[10px] font-bold text-[#7db9c8]", "SMILES NORMALIZATION" }
                            h2 { class: "mt-2 mb-2 text-lg font-bold text-white", "SMILES canonicalizer" }
                            p { class: "m-0 text-[13px] leading-5 text-[#9caabd]",
                                "Generate canonical, isomeric, and kekulized SMILES and inspect hydrogen count and formal charge."
                            }
                        }
                        div { class: "mt-auto flex items-center justify-between border-t border-white/8 pt-4 text-xs font-semibold text-[#8ec4d1]",
                            span { "Open tool" }
                            span { class: "text-base transition-transform group-hover:translate-x-1", ">" }
                        }
                    }

                    Link {
                        class: "group flex min-h-[292px] flex-col rounded-lg border border-[#4b3f34] bg-[#0a1524]/80 p-5 no-underline transition-colors hover:border-[#806b45] hover:bg-[#0d1b2d]",
                        to: crate::route::Route::CheckPains {},
                        div { class: "flex items-start justify-between gap-4",
                            div { class: "grid h-16 w-16 shrink-0 place-items-center rounded-lg border border-[#9b6b8080] bg-[#241823] p-2.5",
                                MoleculeCardIcon { class: "h-full w-full opacity-80", label: "Chemical structure" }
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
                    "Available cheminformatics tools are powered by Rust and execute locally in your browser through WebAssembly."
                }
            }
        }
    }
}
