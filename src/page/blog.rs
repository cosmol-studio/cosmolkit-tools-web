use dioxus::prelude::*;

use crate::component::Seo;

const RUST_CHEMINFORMATICS_ARTICLE: &str =
    include_str!(concat!(env!("OUT_DIR"), "/rust_cheminformatics.html"));
const RUST_CHEMINFORMATICS_STATE_MANAGEMENT_ARTICLE: &str = include_str!(concat!(
    env!("OUT_DIR"),
    "/rust_cheminformatics_state_management.html"
));
const RUST_CHEMINFORMATICS_SOURCE_PORTING_ARTICLE: &str = include_str!(concat!(
    env!("OUT_DIR"),
    "/rust_cheminformatics_source_porting.html"
));
const RUST_CHEMINFORMATICS_VALIDATION_ARTICLE: &str = include_str!(concat!(
    env!("OUT_DIR"),
    "/rust_cheminformatics_validation.html"
));

#[component]
pub fn Blog() -> Element {
    rsx! {
        Seo {
            title: "COSMolKit Blog — Rust Cheminformatics Notes",
            description: "Read COSMolKit articles about Rust cheminformatics, molecular state management, source-backed RDKit ports, and large-scale validation evidence.",
            canonical: "https://tools.cosmol.org/blog",
        }
        div { class: "min-h-screen uu-backdrop m-0 pt-[74px]",
            main { class: "mx-auto w-full max-w-6xl px-6 py-10 font-sans text-[#e8edf5] max-[640px]:px-3.5 max-[640px]:py-7",
                header { class: "max-w-[760px] border-b border-white/10 pb-7",
                    span { class: "text-xs font-bold text-[#4b96ff]", "COSMOLKIT BLOG" }
                    h1 { class: "mb-3 mt-2 text-[32px] leading-tight font-bold text-white max-[640px]:text-[27px]", "Rust cheminformatics notes" }
                    p { class: "m-0 text-[15px] leading-6 text-[#9caabd]",
                        "Technical articles about building, evaluating, and validating open-source molecular software in Rust. Read the complete series from API architecture through validation evidence."
                    }
                }

                section { class: "mt-8 grid grid-cols-2 gap-4 max-[760px]:grid-cols-1", aria_label: "Published articles",
                    BlogCard {
                        route: crate::route::Route::RustCheminformatics {},
                        category: "OVERVIEW",
                        title: "Rust Cheminformatics Beyond RDKit Bindings",
                        summary: "Redesigning molecular APIs around value semantics, explicit mutation, ownership, and safe chemistry state boundaries.",
                        published: true,
                    }
                    BlogCard {
                        route: crate::route::Route::RdkitAlternativeRust {},
                        category: "STATE SEMANTICS",
                        title: "Rust Cheminformatics State Management",
                        summary: "Operation contracts, in-place mutation, cache invalidation, state migration, and semantic safety.",
                        published: true,
                    }
                    BlogCard {
                        route: crate::route::Route::RustCheminformaticsLibraries {},
                        category: "SOURCE PORTING",
                        title: "Rust Cheminformatics Porting",
                        summary: "Why COSMolKit ports pinned RDKit source semantics instead of fitting reimplementations to an output corpus.",
                        published: true,
                    }
                    BlogCard {
                        route: crate::route::Route::Validation {},
                        category: "VALIDATION",
                        title: "Rust Cheminformatics Validation",
                        summary: "From ChEMBL 37 to billions of exact RDKit comparisons across algorithms, state, RNG, batch, and concurrency.",
                        published: true,
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
    published: bool,
) -> Element {
    rsx! {
        Link {
            class: "group flex min-h-[210px] flex-col rounded-lg border border-[#28415f] bg-[#0b1727] p-5 no-underline transition-colors hover:border-[#438ee9] hover:bg-[#0d1b2d]",
            to: route,
            div { class: "flex items-start justify-between gap-4",
                span { class: "text-[10px] font-bold text-[#72bddb]", "{category}" }
                span {
                    class: if published { "rounded-[5px] border border-[#245943] bg-[#102b21] px-2 py-1 text-[10px] font-bold text-[#72d8aa]" } else { "rounded-[5px] border border-[#4b4568] bg-[#1c1930] px-2 py-1 text-[10px] font-bold text-[#ab9de0]" },
                    if published { "PUBLISHED" } else { "PLANNED" }
                }
            }
            h2 { class: "mb-2 mt-5 text-xl font-bold text-white", "{title}" }
            p { class: "m-0 text-[13px] leading-5 text-[#9caabd]", "{summary}" }
            div { class: "mt-auto flex items-center justify-between border-t border-white/8 pt-4 text-xs font-semibold text-[#7ab5ff]",
                span { if published { "Read article" } else { "View article status" } }
                span { class: "text-base transition-transform group-hover:translate-x-1", ">" }
            }
        }
    }
}

#[component]
pub fn RustCheminformatics() -> Element {
    rsx! {
        Seo {
            title: "Rust Cheminformatics Beyond RDKit Bindings | COSMolKit",
            description: "Rust cheminformatics preserves RDKit chemistry without cloning its architecture. COSMolKit uses value semantics, explicit mutation, and operation contracts.",
            canonical: "https://tools.cosmol.org/rust-cheminformatics",
        }
        div { class: "min-h-screen uu-backdrop m-0 pt-[74px]",
            main { class: "mx-auto w-full max-w-4xl px-6 py-10 font-sans text-[#e8edf5] max-[640px]:px-3.5 max-[640px]:py-7",
                Link { class: "text-[13px] font-semibold text-[#7ab5ff] no-underline hover:text-[#b4d6ff]", to: crate::route::Route::Blog {}, "Back to blog" }
                PublishedBlogArticle { html: RUST_CHEMINFORMATICS_ARTICLE }
            }
        }
    }
}

#[component]
pub fn RdkitAlternativeRust() -> Element {
    rsx! {
        Seo {
            title: "Rust Cheminformatics State Management and Molecular Mutation | COSMolKit",
            description: "Learn how COSMolKit uses operation contracts, strict CI, and source-backed ports to make AI-assisted Rust cheminformatics state management safer.",
            canonical: "https://tools.cosmol.org/rdkit-alternative-rust",
        }
        div { class: "min-h-screen uu-backdrop m-0 pt-[74px]",
            main { class: "mx-auto w-full max-w-4xl px-6 py-10 font-sans text-[#e8edf5] max-[640px]:px-3.5 max-[640px]:py-7",
                Link { class: "text-[13px] font-semibold text-[#7ab5ff] no-underline hover:text-[#b4d6ff]", to: crate::route::Route::Blog {}, "Back to blog" }
                PublishedBlogArticle { html: RUST_CHEMINFORMATICS_STATE_MANAGEMENT_ARTICLE }
            }
        }
    }
}

#[component]
pub fn RustCheminformaticsLibraries() -> Element {
    rsx! {
        Seo {
            title: "Rust Cheminformatics: Porting RDKit Source Semantics to Rust | COSMolKit",
            description: "Learn why COSMolKit uses source-backed Rust cheminformatics ports to preserve RDKit semantics, trace mismatches upstream, and retain exact regression evidence.",
            canonical: "https://tools.cosmol.org/rust-cheminformatics-libraries",
        }
        div { class: "min-h-screen uu-backdrop m-0 pt-[74px]",
            main { class: "mx-auto w-full max-w-4xl px-6 py-10 font-sans text-[#e8edf5] max-[640px]:px-3.5 max-[640px]:py-7",
                Link { class: "text-[13px] font-semibold text-[#7ab5ff] no-underline hover:text-[#b4d6ff]", to: crate::route::Route::Blog {}, "Back to blog" }
                PublishedBlogArticle { html: RUST_CHEMINFORMATICS_SOURCE_PORTING_ARTICLE }
            }
        }
    }
}

#[component]
fn PublishedBlogArticle(html: String) -> Element {
    rsx! {
        article {
            class: "mt-7 border-t border-white/10 pt-8 text-[15px] leading-7 text-[#aebacd] [&_h1]:mb-5 [&_h1]:mt-0 [&_h1]:text-[36px] [&_h1]:leading-[1.15] [&_h1]:font-bold [&_h1]:text-white max-[640px]:[&_h1]:text-[29px] [&_h2]:mb-3 [&_h2]:mt-12 [&_h2]:border-t [&_h2]:border-white/8 [&_h2]:pt-8 [&_h2]:text-[24px] [&_h2]:leading-tight [&_h2]:font-bold [&_h2]:text-slate-50 [&_h3]:mb-2 [&_h3]:mt-8 [&_h3]:text-lg [&_h3]:font-bold [&_h3]:text-[#dbe8f8] [&_p]:my-5 [&_ul]:my-5 [&_ul]:list-disc [&_ul]:space-y-1.5 [&_ul]:pl-6 [&_strong]:font-semibold [&_strong]:text-slate-50 [&_a]:font-medium [&_a]:text-[#7ab5ff] [&_a]:underline [&_a]:decoration-[#7ab5ff]/35 [&_a]:underline-offset-4 hover:[&_a]:text-white [&_blockquote]:my-6 [&_blockquote]:border-l-2 [&_blockquote]:border-[#4b96ff] [&_blockquote]:bg-[#0b1727] [&_blockquote]:px-5 [&_blockquote]:py-1 [&_blockquote]:text-[#c5d4e8] [&_pre]:my-6 [&_pre]:max-w-full [&_pre]:overflow-x-auto [&_pre]:rounded-md [&_pre]:border [&_pre]:border-[#263a54] [&_pre]:bg-[#07111f] [&_pre]:p-4 [&_pre]:text-[13px] [&_pre]:leading-6 [&_pre]:text-[#c8d5e6] [&_:not(pre)>code]:rounded-[4px] [&_:not(pre)>code]:bg-[#142239] [&_:not(pre)>code]:px-1.5 [&_:not(pre)>code]:py-0.5 [&_:not(pre)>code]:font-mono [&_:not(pre)>code]:text-[13px] [&_:not(pre)>code]:text-[#b9d8ff] [&_table]:my-6 [&_table]:block [&_table]:w-full [&_table]:overflow-x-auto [&_table]:border-collapse [&_table]:text-[13px] [&_table]:leading-5 [&_th]:min-w-[220px] [&_th]:border [&_th]:border-[#2a3d57] [&_th]:bg-[#0b1727] [&_th]:p-3 [&_th]:text-left [&_th]:font-semibold [&_th]:text-slate-50 [&_td]:min-w-[220px] [&_td]:border [&_td]:border-[#2a3d57] [&_td]:p-3 [&_hr]:my-10 [&_hr]:border-white/8",
            dangerous_inner_html: html,
        }
        nav { class: "mt-12 flex flex-wrap gap-4 border-t border-white/8 pt-5 text-xs", aria_label: "Related pages",
            Link { class: "font-semibold text-[#7ab5ff] no-underline hover:text-white", to: crate::route::Route::Blog {}, "All articles" }
            Link { class: "font-semibold text-[#7ab5ff] no-underline hover:text-white", to: crate::route::Route::Ecosystem {}, "COSMol ecosystem" }
            Link { class: "font-semibold text-[#7ab5ff] no-underline hover:text-white", to: crate::route::Route::ToolDirectory {}, "Browser tools" }
        }
    }
}

#[component]
pub fn Validation() -> Element {
    rsx! {
        Seo {
            title: "Rust Cheminformatics Validation on ChEMBL 37 | COSMolKit",
            description: "See how COSMolKit validates Rust cheminformatics against ChEMBL 37 with exact RDKit comparisons across molecular state, algorithms, RNG, batch, and concurrency.",
            canonical: "https://tools.cosmol.org/validation",
        }
        div { class: "min-h-screen uu-backdrop m-0 pt-[74px]",
            main { class: "mx-auto w-full max-w-4xl px-6 py-10 font-sans text-[#e8edf5] max-[640px]:px-3.5 max-[640px]:py-7",
                Link { class: "text-[13px] font-semibold text-[#7ab5ff] no-underline hover:text-[#b4d6ff]", to: crate::route::Route::Blog {}, "Back to blog" }
                PublishedBlogArticle { html: RUST_CHEMINFORMATICS_VALIDATION_ARTICLE }
            }
        }
    }
}
