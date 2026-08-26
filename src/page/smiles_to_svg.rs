use base64::{Engine as _, engine::general_purpose::STANDARD};
use cosmolkit::Molecule;
use dioxus::prelude::*;

use crate::component::{MdiIcon, Seo, ToastManager, icon::MDI_OPEN_IN_NEW};

const DEFAULT_SMILES: &str = "Cn1c(=O)c2c(ncn2C)n(C)c1=O";
const EXAMPLES: [(&str, &str); 4] = [
    ("Caffeine", DEFAULT_SMILES),
    ("Aspirin", "CC(=O)Oc1ccccc1C(=O)O"),
    ("Serotonin", "NCCc1c[nH]c2ccc(O)cc12"),
    ("Benzene", "c1ccccc1"),
];

#[derive(Clone, PartialEq)]
struct RenderedMolecule {
    svg: String,
    download_url: String,
    atom_count: usize,
    bond_count: usize,
}

fn render_smiles(smiles: &str, width: u32, height: u32) -> Result<RenderedMolecule, String> {
    let smiles = smiles.trim();
    if smiles.is_empty() {
        return Err("Enter a SMILES string to render.".to_string());
    }
    if smiles.len() > 4096 {
        return Err("SMILES input is too long (maximum 4096 characters).".to_string());
    }

    let molecule = Molecule::from_smiles(smiles)
        .map_err(|error| format!("Could not parse this SMILES: {error}"))?;
    let atom_count = molecule.num_atoms();
    let bond_count = molecule.num_bonds();
    let svg = molecule
        .to_svg(width, height)
        .map_err(|error| format!("Could not draw this molecule: {error}"))?;
    let download_url = svg_data_url(&svg);

    Ok(RenderedMolecule {
        svg,
        download_url,
        atom_count,
        bond_count,
    })
}

fn svg_data_url(svg: &str) -> String {
    format!(
        "data:image/svg+xml;base64,{}",
        STANDARD.encode(svg.as_bytes())
    )
}

fn python_string_literal(value: &str) -> String {
    let mut literal = String::with_capacity(value.len() + 2);
    literal.push('"');
    for character in value.chars() {
        match character {
            '"' => literal.push_str("\\\""),
            '\\' => literal.push_str("\\\\"),
            '\n' => literal.push_str("\\n"),
            '\r' => literal.push_str("\\r"),
            '\t' => literal.push_str("\\t"),
            character if character.is_control() => {
                use std::fmt::Write;
                let _ = write!(literal, "\\u{:04x}", character as u32);
            }
            character => literal.push(character),
        }
    }
    literal.push('"');
    literal
}

fn python_example(smiles: &str, width: u32, height: u32) -> String {
    format!(
        "from pathlib import Path\nfrom cosmolkit import Molecule\n\n\
         smiles = {}\n\
         mol = Molecule.from_smiles(smiles).with_2d_coordinates()\n\
         svg = mol.to_svg(width={width}, height={height})\n\n\
         Path(\"molecule.svg\").write_text(svg, encoding=\"utf-8\")",
        python_string_literal(smiles.trim()),
    )
}

fn update_render(
    smiles: String,
    width: u32,
    height: u32,
    mut rendered: Signal<Option<Result<RenderedMolecule, String>>>,
) {
    rendered.set(Some(render_smiles(&smiles, width, height)));
}

#[cfg(target_arch = "wasm32")]
fn copy_text(text: String, success_message: &'static str, mut toast: ToastManager) {
    wasm_bindgen_futures::spawn_local(async move {
        let result = async {
            let window = web_sys::window().ok_or("Browser window is unavailable.")?;
            let clipboard = window.navigator().clipboard();
            wasm_bindgen_futures::JsFuture::from(clipboard.write_text(&text))
                .await
                .map_err(|_| "Clipboard access was denied.")?;
            Ok::<(), &str>(())
        }
        .await;

        match result {
            Ok(()) => toast.success(success_message),
            Err(message) => toast.error(message),
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn copy_text(_text: String, _success_message: &'static str, mut toast: ToastManager) {
    toast.info("Clipboard export is available in the web build.");
}

#[component]
pub fn SmilesToSvg() -> Element {
    let mut smiles = use_signal(|| DEFAULT_SMILES.to_string());
    let mut width = use_signal(|| 720_u32);
    let mut height = use_signal(|| 480_u32);
    let mut rendered = use_signal(|| None::<Result<RenderedMolecule, String>>);
    let toast = use_context::<ToastManager>();
    let cosmolkit_version = cosmolkit::version();
    let python_code = python_example(&smiles(), width(), height());

    use_effect(move || {
        rendered.set(Some(render_smiles(DEFAULT_SMILES, 720, 480)));
    });

    rsx! {
        Seo {
            title: "SMILES to SVG — Molecular Structure Renderer | COSMolKit",
            description: "Use this online SMILES renderer to create scalable SVG chemical structure drawings with COSMolKit's Rust cheminformatics core, locally in your browser.",
            canonical: "https://tools.cosmol.org/smiles-to-svg",
        }
        div {
            class: "uu-backdrop m-0 pt-[74px]",
            main{
                class: "max-w-6xl mx-auto py-5 font-sans text-[#e8edf5] max-[800px]:px-3.5 max-[800px]:pb-[30px]",
                section { class: "mx-auto w-full max-w-[1180px]",
                    div { class: "mb-6 flex items-end justify-between gap-8 max-[800px]:flex-col max-[800px]:items-start max-[800px]:gap-4",
                        div {
                            Link {
                                class: "text-[13px] font-semibold text-[#7ab5ff] no-underline hover:text-[#b4d6ff]",
                                to: crate::route::Route::ToolDirectory {},
                                "Back to tools"
                            }
                            h1 { class: "mb-1.5 mt-2.5 text-[32px] leading-[1.2] font-bold text-slate-50 max-[800px]:text-[27px]", "SMILES to SVG" }
                            p { class: "m-0 max-w-[620px] text-[15px] leading-[1.6] text-[#9caabd]", "Generate publication-ready 2D molecular structures entirely in your browser." }
                    }
                    a {
                        class: "inline-flex shrink-0 items-center gap-2 rounded-md border border-[#23344a] bg-[#0c1828] px-[11px] py-2 text-xs font-semibold text-[#b8c5d6] no-underline hover:border-[#438ee9] hover:text-white",
                        href: "https://crates.io/crates/cosmolkit",
                        target: "_blank",
                        rel: "noreferrer",
                        "COSMolKit {cosmolkit_version} / Rust / WASM"
                        MdiIcon { size: 14, path: MDI_OPEN_IN_NEW }
                        }
                    }

                    div { class: "grid min-h-[650px] grid-cols-[340px_minmax(0,1fr)] overflow-hidden rounded-lg border border-[#213147] bg-[#0c1727] shadow-[0_20px_55px_rgba(0,0,0,0.24)] max-[800px]:grid-cols-1",
                        aside { class: "border-r border-[#213147] bg-[#0a1524] p-6 max-[800px]:border-r-0 max-[800px]:border-b max-[480px]:p-[18px]",
                            div { class: "mb-6",
                                div { class: "flex items-center justify-between",
                                    label { class: "mb-[9px] block text-[13px] font-bold text-[#dce5f0]", r#for: "smiles-input", "SMILES" }
                                    span { class: "text-[11px] text-[#6f8095]", "{smiles().len()} / 4096" }
                                }
                                textarea {
                                    class: "block min-h-[138px] w-full resize-y rounded-md border border-[#2a3b52] bg-[#07111f] px-3.5 py-[13px] font-mono text-[13px] leading-[1.65] text-[#eaf1fa] outline-none hover:border-[#438ee9] focus:border-[#438ee9] focus:ring-[3px] focus:ring-[#3082ff1f]",
                                    id: "smiles-input",
                                    value: "{smiles}",
                                    maxlength: 4096,
                                    spellcheck: false,
                                    oninput: move |event| smiles.set(event.value()),
                                }
                            }

                            div { class: "mb-6",
                                label { class: "mb-[9px] block text-[13px] font-bold text-[#dce5f0]", "Examples" }
                                div { class: "flex flex-wrap gap-[7px]",
                                    for (name, value) in EXAMPLES {
                                        button {
                                            r#type: "button",
                                            class: "cursor-pointer rounded-[5px] border border-[#2b3d54] bg-[#101e30] px-[9px] py-1.5 text-xs text-[#adbbcc] hover:border-[#438ee9] hover:text-[#eaf3ff]",
                                            onclick: move |_| {
                                                smiles.set(value.to_string());
                                                update_render(
                                                    value.to_string(),
                                                    width(),
                                                    height(),
                                                    rendered,
                                                );
                                            },
                                            "{name}"
                                        }
                                    }
                                }
                            }

                            div { class: "mb-6",
                                label { class: "mb-[9px] block text-[13px] font-bold text-[#dce5f0]", "Canvas size" }
                                div { class: "grid grid-cols-2 gap-2.5",
                                    div { class: "relative",
                                        span { class: "pointer-events-none absolute top-[9px] left-[11px] text-[10px] font-bold text-[#718299]", "Width" }
                                        input {
                                            class: "h-[54px] w-full rounded-md border border-[#2a3b52] bg-[#07111f] px-2.5 pt-[23px] pb-1.5 text-[13px] text-[#eaf1fa] outline-none hover:border-[#438ee9] focus:border-[#438ee9] focus:ring-[3px] focus:ring-[#3082ff1f]",
                                            r#type: "number",
                                            min: 160,
                                            max: 2000,
                                            step: 10,
                                            value: "{width}",
                                            oninput: move |event| {
                                                if let Ok(value) = event.value().parse::<u32>() {
                                                    width.set(value.clamp(160, 2000));
                                                }
                                            },
                                        }
                                    }
                                    div { class: "relative",
                                        span { class: "pointer-events-none absolute top-[9px] left-[11px] text-[10px] font-bold text-[#718299]", "Height" }
                                        input {
                                            class: "h-[54px] w-full rounded-md border border-[#2a3b52] bg-[#07111f] px-2.5 pt-[23px] pb-1.5 text-[13px] text-[#eaf1fa] outline-none hover:border-[#438ee9] focus:border-[#438ee9] focus:ring-[3px] focus:ring-[#3082ff1f]",
                                            r#type: "number",
                                            min: 160,
                                            max: 2000,
                                            step: 10,
                                            value: "{height}",
                                            oninput: move |event| {
                                                if let Ok(value) = event.value().parse::<u32>() {
                                                    height.set(value.clamp(160, 2000));
                                                }
                                            },
                                        }
                                    }
                                }
                            }

                            button {
                                r#type: "button",
                                class: "h-[42px] w-full cursor-pointer rounded-md border border-[#398cf6] bg-[#267de8] text-[13px] font-bold text-white shadow-[0_8px_24px_rgba(38,125,232,0.2)] hover:bg-[#358bf3] active:translate-y-px",
                                onclick: move |_| {
                                    update_render(smiles(), width(), height(), rendered);
                                },
                                "Render molecule"
                            }

                            p { class: "mt-[13px] mb-0 flex items-center justify-center gap-[7px] text-[11px] text-[#718299]",
                                span { class: "inline-grid h-[15px] w-[15px] place-items-center rounded-full border border-[#3a806d] text-[10px] text-[#54c7a0]", "*" }
                                "Your molecule never leaves this device."
                            }
                        }

                        section { class: "grid min-w-0 grid-rows-[auto_1fr] bg-[#111c2c]",
                            div { class: "flex min-h-16 items-center justify-between gap-5 border-b border-[#213147] px-[18px] py-3 max-[800px]:flex-col max-[800px]:items-start",
                                div {
                                    h2 { class: "mt-0 mb-[3px] text-sm leading-[1.2] font-bold text-[#eef4fb]", "Preview" }
                                    if let Some(Ok(result)) = &*rendered.read() {
                                        span { class: "text-[11px] text-[#718299]", "{result.atom_count} atoms  /  {result.bond_count} bonds" }
                                    }
                                }
                                if let Some(Ok(result)) = &*rendered.read() {
                                    div { class: "flex gap-2 max-[480px]:w-full",
                                        button {
                                            r#type: "button",
                                            class: "inline-flex h-[34px] cursor-pointer items-center justify-center rounded-[5px] border border-[#30435b] bg-[#0c1828] px-[11px] text-xs font-semibold text-[#c5d1df] hover:border-[#438ee9] hover:text-white max-[480px]:flex-1",
                                            title: "Copy SVG source",
                                        onclick: {
                                            let svg = result.svg.clone();
                                            move |_| {
                                                copy_text(
                                                    svg.clone(),
                                                    "SVG source copied.",
                                                    toast,
                                                )
                                            }
                                            },
                                            "Copy SVG"
                                        }
                                        a {
                                            class: "inline-flex h-[34px] cursor-pointer items-center justify-center rounded-[5px] border border-[#30435b] bg-[#0c1828] px-[11px] text-xs leading-none font-semibold text-[#c5d1df] no-underline hover:border-[#438ee9] hover:text-white max-[480px]:flex-1",
                                            href: result.download_url.clone(),
                                            download: "molecule.svg",
                                            "Download"
                                        }
                                    }
                                }
                            }

                            div { id: "svg-preview-stage", class: "grid min-h-[560px] place-items-center bg-[#e9eef4] p-6 max-[800px]:min-h-[420px] max-[800px]:p-3",
                                match &*rendered.read() {
                                    Some(Ok(result)) => rsx! {
                                        div {
                                            id: "svg-output",
                                            class: "grid h-full min-h-[480px] w-full place-items-center overflow-hidden border border-[#d5dde7] bg-white max-[800px]:min-h-[380px]",
                                            img {
                                                class: "block h-full max-h-full w-full max-w-full object-contain",
                                                src: result.download_url.clone(),
                                                alt: "2D molecular structure generated from SMILES",
                                            }
                                        }
                                    },
                                    Some(Err(error)) => rsx! {
                                        div { class: "max-w-[480px] text-center",
                                            div { class: "mx-auto mt-0 mb-3.5 grid h-[42px] w-[42px] place-items-center rounded-full border border-[#e6a9a9] bg-[#fff5f5] text-lg font-extrabold text-[#c83e3e]", "!" }
                                            h3 { class: "mt-0 mb-[7px] text-base font-bold text-[#172234]", "Unable to render" }
                                            p { class: "m-0 text-[13px] leading-[1.55] text-[#68778a]", "{error}" }
                                        }
                                    },
                                    None => rsx! {
                                        div { class: "max-w-[480px] text-center",
                                            div { class: "mx-auto mb-3 h-8 w-8 animate-spin rounded-full border-2 border-[#b8c7d8] border-t-[#267de8]" }
                                            p { class: "m-0 text-[13px] text-[#68778a]", "Preparing molecular preview" }
                                        }
                                    },
                                }
                            }

                        }
                    }
                }

                section { class: "mt-10 border-t border-[#213147] pt-8",
                    span { class: "text-xs font-bold text-[#4b96ff]", "BROWSER-LOCAL DEPICTION" }
                    h2 { class: "mb-2 mt-2 text-xl font-bold text-slate-50", "Convert SMILES to an SVG molecular structure" }
                    p { class: "m-0 max-w-[820px] text-sm leading-6 text-[#9caabd]",
                        "COSMolKit parses the SMILES, generates 2D coordinates, and renders the molecular structure as SVG on this device. This browser-local SMILES visualizer and molecule drawing tool produces a vector graphic that can be copied or downloaded without uploading the structure to a server."
                    }
                }

                section { class: "mt-10 border-t border-[#213147] pt-8",
                    div { class: "flex items-start justify-between gap-6 max-[800px]:flex-col",
                        div {
                            span { class: "text-xs font-bold text-[#4b96ff]", "PYTHON BACKEND" }
                            h2 { class: "mt-2 mb-1.5 text-xl font-bold text-slate-50", "Build with the same COSMolKit core" }
                            p { class: "m-0 max-w-[680px] text-sm leading-6 text-[#9caabd]",
                                "The browser tool and Python package share the same Rust molecular graph, 2D coordinate generation, and SVG renderer."
                            }
                        }
                        a {
                            class: "inline-flex shrink-0 items-center gap-2 rounded-md border border-[#2a3b52] bg-[#0c1828] px-3 py-2 text-xs font-semibold text-[#b8c5d6] no-underline hover:border-[#438ee9] hover:text-white",
                            href: "https://pypi.org/project/cosmolkit/{cosmolkit_version}/",
                            target: "_blank",
                            rel: "noreferrer",
                            "COSMolKit Python {cosmolkit_version}"
                            MdiIcon { size: 14, path: MDI_OPEN_IN_NEW }
                        }
                    }

                    div { class: "mt-6 grid grid-cols-[240px_minmax(0,1fr)] gap-6 max-[800px]:grid-cols-1",
                        div { class: "border-l-2 border-[#267de8] pl-4",
                            span { class: "block text-[11px] font-bold text-[#718299]", "INSTALL" }
                            code { class: "mt-2 block break-all font-mono text-[13px] text-[#dce5f0]",
                                "pip install cosmolkit=={cosmolkit_version}"
                            }
                            p { class: "mt-3 mb-0 text-xs leading-5 text-[#718299]",
                                "Python 3.9+ / Rust-native wheel"
                            }
                        }

                        div { class: "min-w-0 overflow-hidden rounded-lg border border-[#213147] bg-[#081321]",
                            div { class: "flex min-h-11 items-center justify-between border-b border-[#213147] px-4",
                                div { class: "flex items-center gap-2",
                                    span { class: "h-2 w-2 rounded-full bg-[#f0c35a]" }
                                    span { class: "text-xs font-semibold text-[#9caabd]", "generate_svg.py" }
                                }
                                button {
                                    r#type: "button",
                                    class: "cursor-pointer rounded-[5px] border border-[#30435b] bg-[#0c1828] px-2.5 py-1.5 text-[11px] font-semibold text-[#c5d1df] hover:border-[#438ee9] hover:text-white",
                                    onclick: {
                                        let python_code = python_code.clone();
                                        move |_| {
                                            copy_text(
                                                python_code.clone(),
                                                "Python example copied.",
                                                toast,
                                            )
                                        }
                                    },
                                    "Copy Python"
                                }
                            }
                            pre { class: "m-0 overflow-x-auto p-4 font-mono text-[13px] leading-6 text-[#d6e2f0]",
                                code { "{python_code}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_valid_smiles_as_svg() {
        let output = render_smiles("c1ccccc1", 400, 300).unwrap();
        assert!(output.svg.contains("<svg"));
        assert_eq!(output.atom_count, 6);
        assert_eq!(output.bond_count, 6);
        let encoded = output
            .download_url
            .strip_prefix("data:image/svg+xml;base64,")
            .expect("SVG should use a base64 data URL");
        assert_eq!(STANDARD.decode(encoded).unwrap(), output.svg.as_bytes());
    }

    #[test]
    fn rejects_invalid_smiles() {
        assert!(render_smiles("C(", 400, 300).is_err());
    }

    #[test]
    fn python_example_matches_current_render_settings() {
        let example = python_example("C\\\"N", 640, 360);
        assert!(example.contains("Molecule.from_smiles"));
        assert!(example.contains("to_svg(width=640, height=360)"));
        assert!(example.contains(r#"smiles = "C\\\"N""#));
    }
}
