use dioxus::prelude::*;

use crate::component::Seo;

#[component]
pub fn Blog() -> Element {
    rsx! {
        Seo {
            title: "COSMolKit Blog — Rust Cheminformatics Notes",
            description: "Upcoming technical articles about Rust cheminformatics, molecular software validation, and the COSMol open-source ecosystem.",
            canonical: "https://tools.cosmol.org/blog",
        }
        document::Meta { name: "robots", content: "noindex, follow" }
        div { class: "min-h-screen uu-backdrop m-0 pt-[74px]",
            main { class: "mx-auto w-full max-w-6xl px-6 py-10 font-sans text-[#e8edf5] max-[640px]:px-3.5 max-[640px]:py-7",
                header { class: "max-w-[760px] border-b border-white/10 pb-7",
                    span { class: "text-xs font-bold text-[#4b96ff]", "COSMOLKIT BLOG" }
                    h1 { class: "mb-3 mt-2 text-[32px] leading-tight font-bold text-white max-[640px]:text-[27px]", "Rust cheminformatics notes" }
                    p { class: "m-0 text-[15px] leading-6 text-[#9caabd]",
                        "Technical articles about building, evaluating, and validating open-source molecular software in Rust. The first articles are currently in preparation."
                    }
                }

                section { class: "mt-8 grid grid-cols-2 gap-4 max-[760px]:grid-cols-1", aria_label: "Planned articles",
                    BlogCard {
                        route: crate::route::Route::RustCheminformatics {},
                        category: "OVERVIEW",
                        title: "Rust cheminformatics",
                        summary: "An introduction to the current Rust ecosystem for molecular data and cheminformatics workflows.",
                    }
                    BlogCard {
                        route: crate::route::Route::RdkitAlternativeRust {},
                        category: "COMPARISON",
                        title: "A Rust alternative to RDKit?",
                        summary: "A scoped look at where native Rust chemistry libraries fit, including capabilities and current limitations.",
                    }
                    BlogCard {
                        route: crate::route::Route::RustCheminformaticsLibraries {},
                        category: "ECOSYSTEM",
                        title: "Rust cheminformatics libraries",
                        summary: "A practical map of open-source Rust crates for molecular representations, formats, algorithms, and visualization.",
                    }
                    BlogCard {
                        route: crate::route::Route::Validation {},
                        category: "ENGINEERING",
                        title: "Validation",
                        summary: "How molecular software behavior can be checked with fixtures, reference data, and cross-implementation tests.",
                    }
                }
            }
        }
    }
}

#[component]
fn BlogCard(
    route: crate::route::Route,
    category: String,
    title: String,
    summary: String,
) -> Element {
    rsx! {
        Link {
            class: "group flex min-h-[210px] flex-col rounded-lg border border-[#28415f] bg-[#0b1727] p-5 no-underline transition-colors hover:border-[#438ee9] hover:bg-[#0d1b2d]",
            to: route,
            div { class: "flex items-start justify-between gap-4",
                span { class: "text-[10px] font-bold text-[#72bddb]", "{category}" }
                span { class: "rounded-[5px] border border-[#4b4568] bg-[#1c1930] px-2 py-1 text-[10px] font-bold text-[#ab9de0]", "PLANNED" }
            }
            h2 { class: "mb-2 mt-5 text-xl font-bold text-white", "{title}" }
            p { class: "m-0 text-[13px] leading-5 text-[#9caabd]", "{summary}" }
            div { class: "mt-auto flex items-center justify-between border-t border-white/8 pt-4 text-xs font-semibold text-[#7ab5ff]",
                span { "View article status" }
                span { class: "text-base transition-transform group-hover:translate-x-1", ">" }
            }
        }
    }
}

#[component]
pub fn RustCheminformatics() -> Element {
    rsx! {
        BlogPlaceholder {
            title: "Rust Cheminformatics — COSMolKit Blog",
            description: "A planned technical overview of Rust cheminformatics libraries, molecular data workflows, and the open-source ecosystem.",
            canonical: "https://tools.cosmol.org/rust-cheminformatics",
            category: "ECOSYSTEM OVERVIEW",
            heading: "Rust cheminformatics",
            summary: "This article will introduce the Rust ecosystem for molecular representations, file formats, cheminformatics algorithms, and browser-native workflows.",
        }
    }
}

#[component]
pub fn RdkitAlternativeRust() -> Element {
    rsx! {
        BlogPlaceholder {
            title: "RDKit Alternative in Rust — COSMolKit Blog",
            description: "A planned, scoped comparison of native Rust cheminformatics capabilities and established molecular software such as RDKit.",
            canonical: "https://tools.cosmol.org/rdkit-alternative-rust",
            category: "CAPABILITY COMPARISON",
            heading: "A Rust alternative to RDKit?",
            summary: "This article will compare concrete capabilities, integration models, and current limitations without claiming unsupported feature parity.",
        }
    }
}

#[component]
pub fn RustCheminformaticsLibraries() -> Element {
    rsx! {
        BlogPlaceholder {
            title: "Rust Cheminformatics Libraries — COSMolKit Blog",
            description: "A planned guide to Rust libraries for molecular graphs, chemical formats, cheminformatics algorithms, and visualization.",
            canonical: "https://tools.cosmol.org/rust-cheminformatics-libraries",
            category: "LIBRARY GUIDE",
            heading: "Rust cheminformatics libraries",
            summary: "This article will organize relevant Rust crates by responsibility and document what each project currently provides.",
        }
    }
}

#[component]
pub fn Validation() -> Element {
    rsx! {
        BlogPlaceholder {
            title: "Cheminformatics Software Validation — COSMolKit Blog",
            description: "A planned technical article about validating molecular software with fixtures, reference data, and cross-implementation tests.",
            canonical: "https://tools.cosmol.org/validation",
            category: "SOFTWARE VALIDATION",
            heading: "Validating cheminformatics software",
            summary: "This article will cover reproducible validation strategies for parsers, molecular algorithms, descriptors, and coordinate generation.",
        }
    }
}

#[component]
fn BlogPlaceholder(
    title: String,
    description: String,
    canonical: String,
    category: String,
    heading: String,
    summary: String,
) -> Element {
    rsx! {
        Seo { title, description, canonical }
        document::Meta { name: "robots", content: "noindex, follow" }
        div { class: "min-h-screen uu-backdrop m-0 pt-[74px]",
            main { class: "mx-auto w-full max-w-4xl px-6 py-10 font-sans text-[#e8edf5] max-[640px]:px-3.5 max-[640px]:py-7",
                Link { class: "text-[13px] font-semibold text-[#7ab5ff] no-underline hover:text-[#b4d6ff]", to: crate::route::Route::Blog {}, "Back to blog" }
                article { class: "mt-7 border-t border-white/10 pt-8",
                    span { class: "text-xs font-bold text-[#72bddb]", "{category}" }
                    h1 { class: "mb-4 mt-2 text-[34px] leading-tight font-bold text-white max-[640px]:text-[28px]", "{heading}" }
                    p { class: "m-0 max-w-[720px] text-[15px] leading-7 text-[#9caabd]", "{summary}" }
                    div { class: "mt-9 border-l-2 border-[#4b96ff] bg-[#0b1727] px-5 py-4",
                        span { class: "block text-xs font-bold text-[#a9cfff]", "ARTICLE IN PREPARATION" }
                        p { class: "mb-0 mt-2 text-sm leading-6 text-[#8495aa]", "The route is reserved, but the article has not been published yet." }
                    }
                }
                nav { class: "mt-9 flex flex-wrap gap-4 border-t border-white/8 pt-5 text-xs", aria_label: "Related pages",
                    Link { class: "font-semibold text-[#7ab5ff] no-underline hover:text-white", to: crate::route::Route::Blog {}, "All planned articles" }
                    Link { class: "font-semibold text-[#7ab5ff] no-underline hover:text-white", to: crate::route::Route::Ecosystem {}, "COSMol ecosystem" }
                    Link { class: "font-semibold text-[#7ab5ff] no-underline hover:text-white", to: crate::route::Route::ToolDirectory {}, "Browser tools" }
                }
            }
        }
    }
}
