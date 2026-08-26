use dioxus::prelude::*;

use crate::component::Seo;

#[derive(Clone, Copy, PartialEq)]
struct Project {
    name: &'static str,
    role: &'static str,
    description: &'static str,
    href: &'static str,
    accent: &'static str,
    badge: &'static str,
}

const PROJECTS: [Project; 3] = [
    Project {
        name: "COSMolKit",
        role: "Chemical core",
        description: "The Rust cheminformatics toolkit. It owns molecular graphs, SMILES and InChI, file formats, coordinates, and chemistry operations.",
        href: "https://github.com/cosmol-studio/COSMolKit",
        accent: "#4b96ff",
        badge: "CORE",
    },
    Project {
        name: "COSMol-viewer",
        role: "3D visualization",
        description: "The Rust viewer layer for interactive molecular and structural biology scenes, including the WebAssembly canvas used here.",
        href: "https://github.com/cosmol-studio/COSMol-viewer",
        accent: "#ab9de0",
        badge: "VIEWER",
    },
    Project {
        name: "cosmolkit-tools-web",
        role: "User-facing tools",
        description: "This Dioxus web application. It combines the core and viewer into private, browser-native workflows with no backend service required.",
        href: "https://github.com/cosmol-studio/cosmolkit-tools-web",
        accent: "#72bddb",
        badge: "WEB APP",
    },
];

#[component]
pub fn Ecosystem() -> Element {
    rsx! {
        Seo {
            title: "COSMol Ecosystem — Rust-Powered Cheminformatics & Browser-Native Tools",
            description: "Explore the COSMol open-source ecosystem: COSMolKit's Rust cheminformatics core, COSMol-viewer visualization, and browser-native molecular tools.",
            canonical: "https://tools.cosmol.org/ecosystem",
        }
        div { class: "min-h-screen uu-backdrop m-0 pt-[74px]",
            main { class: "mx-auto w-full max-w-6xl px-6 py-10 font-sans text-[#e8edf5] max-[640px]:px-3.5 max-[640px]:py-7",
                header { class: "max-w-[760px]",
                    Link { class: "text-[13px] font-semibold text-[#7ab5ff] no-underline hover:text-[#b4d6ff]", to: crate::route::Route::Home {}, "Back to home" }
                    span { class: "mt-7 block text-xs font-bold tracking-[0.08em] text-[#4b96ff]", "OPEN SOURCE ECOSYSTEM" }
                    h1 { class: "mb-3 mt-2 text-[32px] leading-[1.2] font-bold text-slate-50 max-[640px]:text-[27px]", "One molecular stack, three focused projects." }
                    p { class: "m-0 text-[15px] leading-6 text-[#9caabd]", "The open-source Rust cheminformatics stack: COSMolKit provides the chemistry, COSMol-viewer provides the interactive scene, and this web app turns both into practical browser-native tools." }
                }

                section { class: "mt-10 grid grid-cols-3 gap-4 max-[900px]:grid-cols-1",
                    for project in PROJECTS {
                        article { class: "flex min-h-[280px] flex-col rounded-lg border border-[#213147] bg-[#0b1727] p-6 shadow-[0_16px_40px_rgba(0,0,0,0.16)]",
                            div { class: "flex items-start justify-between gap-4",
                                div {
                                    span { class: "block text-[10px] font-bold tracking-[0.08em]", style: "color: {project.accent}", "{project.badge}" }
                                    h2 { class: "mb-1 mt-2 text-xl font-bold text-white", "{project.name}" }
                                    span { class: "text-xs font-semibold text-[#8495aa]", "{project.role}" }
                                }
                                span { class: "mt-1 h-2.5 w-2.5 shrink-0 rounded-full", style: "background: {project.accent}" }
                            }
                            p { class: "mt-6 text-[13px] leading-6 text-[#9caabd]", "{project.description}" }
                            a { class: "mt-auto inline-flex items-center gap-2 border-t border-white/8 pt-4 text-xs font-semibold no-underline hover:text-white", style: "color: {project.accent}", href: project.href, target: "_blank", rel: "noreferrer", "View repository", span { "->" } }
                        }
                    }
                }

                section { class: "mt-10 border-t border-white/10 pt-8",
                    h2 { class: "mb-5 mt-0 text-xl font-bold text-white", "How the pieces connect" }
                    div { class: "grid grid-cols-[minmax(0,1fr)_54px_minmax(0,1fr)_54px_minmax(0,1fr)] items-center gap-3 max-[760px]:grid-cols-1",
                        div { class: "rounded-md border border-[#28415f] bg-[#0c1828] p-4", span { class: "text-xs font-bold text-[#4b96ff]", "MOLECULAR DATA" } p { class: "mt-2 text-xs leading-5 text-[#9caabd]", "SMILES, InChI, MOL, SDF, PDB, and more." } }
                        span { class: "text-center text-xl text-[#536a86] max-[760px]:rotate-90", "->" }
                        div { class: "rounded-md border border-[#4b4568] bg-[#0c1828] p-4", span { class: "text-xs font-bold text-[#ab9de0]", "COSMOLKIT + VIEWER" } p { class: "mt-2 text-xs leading-5 text-[#9caabd]", "Parse, compute coordinates, and render a scene." } }
                        span { class: "text-center text-xl text-[#536a86] max-[760px]:rotate-90", "->" }
                        div { class: "rounded-md border border-[#315266] bg-[#0c1828] p-4", span { class: "text-xs font-bold text-[#72bddb]", "TOOLS WEB" } p { class: "mt-2 text-xs leading-5 text-[#9caabd]", "Private workflows that run inside your browser." } }
                    }
                }

                footer { class: "mt-10 flex flex-wrap items-center gap-x-5 gap-y-2 border-t border-white/8 pt-5 text-xs text-[#718299]",
                    span { "Built with Rust, Dioxus, and WebAssembly." }
                    Link { class: "font-semibold text-[#7ab5ff] no-underline hover:text-white", to: crate::route::Route::ToolDirectory {}, "Browse all tools ->" }
                }
            }
        }
    }
}
