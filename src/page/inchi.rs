use cosmolkit::{Molecule, inchi_to_inchi_key, mol_from_inchi, mol_to_inchi};
use dioxus::prelude::*;

use crate::component::{MdiIcon, Seo, ToastManager, icon::MDI_OPEN_IN_NEW};

const DEFAULT_SMILES: &str = "CC(=O)Oc1ccccc1C(=O)O";
const DEFAULT_INCHI: &str = "InChI=1S/C9H8O4/c1-6(10)13-8-5-3-2-4-7(8)9(11)12/h2-5H,1H3,(H,11,12)";
const MAX_INPUT_LENGTH: usize = 16_384;
const EXAMPLES: [(&str, &str, &str); 4] = [
    ("Aspirin", DEFAULT_SMILES, DEFAULT_INCHI),
    (
        "Caffeine",
        "Cn1c(=O)c2c(ncn2C)n(C)c1=O",
        "InChI=1S/C8H10N4O2/c1-10-4-9-6(10)11(2)8(14)12(3)7(9)13-5(4)15/h1-3H3",
    ),
    ("Ethanol", "CCO", "InChI=1S/C2H6O/c1-2-3/h3H,2H2,1H3"),
    ("Benzene", "c1ccccc1", "InChI=1S/C6H6/c1-2-4-6-5-3-1/h1-6H"),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum InchiMode {
    FromSmiles,
    ToStructure,
}

#[derive(Clone, Debug, PartialEq)]
struct InchiResult {
    inchi: String,
    inchi_key: String,
    smiles: String,
    svg_url: String,
    atom_count: usize,
    bond_count: usize,
    download_url: String,
}

fn checked_input<'a>(input: &'a str, label: &str) -> Result<&'a str, String> {
    let input = input.trim();
    if input.is_empty() {
        return Err(format!("Enter a {label} value to convert."));
    }
    if input.len() > MAX_INPUT_LENGTH {
        return Err(format!(
            "{label} input is too long (maximum {MAX_INPUT_LENGTH} characters)."
        ));
    }
    Ok(input)
}

fn utf8_output(bytes: Vec<u8>, label: &str) -> Result<String, String> {
    String::from_utf8(bytes).map_err(|_| format!("COSMolKit returned a non-UTF-8 {label}."))
}

fn render_result(
    molecule: Molecule,
    inchi: String,
    inchi_key: String,
) -> Result<InchiResult, String> {
    if inchi.is_empty() || inchi_key.is_empty() {
        return Err("The InChI engine did not return a complete identifier.".to_string());
    }

    let atom_count = molecule.num_atoms();
    let bond_count = molecule.num_bonds();
    let smiles = molecule
        .to_smiles(true)
        .map_err(|error| format!("Could not write canonical SMILES: {error}"))?;
    let svg = molecule
        .to_svg(720, 480)
        .map_err(|error| format!("Could not draw the parsed structure: {error}"))?;

    Ok(InchiResult {
        download_url: data_url("chemical/x-inchi", &format!("{inchi}\n")),
        svg_url: data_url("image/svg+xml", &svg),
        inchi,
        inchi_key,
        smiles,
        atom_count,
        bond_count,
    })
}

fn from_smiles(input: &str) -> Result<InchiResult, String> {
    let smiles = checked_input(input, "SMILES")?;
    let molecule = Molecule::from_smiles(smiles)
        .map_err(|error| format!("Could not parse this SMILES: {error}"))?;
    let generated = mol_to_inchi(&molecule, None)
        .map_err(|error| format!("Could not generate InChI: {error}"))?;
    let inchi = utf8_output(generated.inchi, "InChI")?;
    let key = inchi_to_inchi_key(inchi.as_bytes())
        .map_err(|error| format!("Could not generate InChIKey: {error}"))?;
    let inchi_key = utf8_output(key.key, "InChIKey")?;
    render_result(molecule, inchi, inchi_key)
}

fn to_structure(input: &str) -> Result<InchiResult, String> {
    let inchi = checked_input(input, "InChI")?;
    if !inchi.starts_with("InChI=") {
        return Err("InChI input must start with `InChI=`.".to_string());
    }

    let parsed = mol_from_inchi(inchi.as_bytes(), false, false)
        .map_err(|error| format!("Could not parse this InChI: {error}"))?;
    let molecule = parsed
        .molecule
        .ok_or_else(|| "The InChI engine did not return a molecular structure.".to_string())?;
    let key = inchi_to_inchi_key(inchi.as_bytes())
        .map_err(|error| format!("Could not generate InChIKey: {error}"))?;
    let inchi_key = utf8_output(key.key, "InChIKey")?;
    render_result(molecule, inchi.to_string(), inchi_key)
}

fn convert(input: &str, mode: InchiMode) -> Result<InchiResult, String> {
    match mode {
        InchiMode::FromSmiles => from_smiles(input),
        InchiMode::ToStructure => to_structure(input),
    }
}

fn data_url(mime: &str, content: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut encoded = String::with_capacity(content.len() * 2);
    encoded.push_str("data:");
    encoded.push_str(mime);
    encoded.push_str(";charset=utf-8,");
    for byte in content.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            encoded.push(byte as char);
        } else {
            encoded.push('%');
            encoded.push(HEX[(byte >> 4) as usize] as char);
            encoded.push(HEX[(byte & 0x0f) as usize] as char);
        }
    }
    encoded
}

#[cfg(target_arch = "wasm32")]
fn copy_text(text: String, success_message: &'static str, mut toast: ToastManager) {
    wasm_bindgen_futures::spawn_local(async move {
        let outcome = async {
            let window = web_sys::window().ok_or("Browser window is unavailable.")?;
            wasm_bindgen_futures::JsFuture::from(window.navigator().clipboard().write_text(&text))
                .await
                .map_err(|_| "Clipboard access was denied.")?;
            Ok::<(), &str>(())
        }
        .await;
        match outcome {
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
pub fn InchiTool() -> Element {
    let mut mode = use_signal(|| InchiMode::FromSmiles);
    let mut input = use_signal(|| DEFAULT_SMILES.to_string());
    let mut result = use_signal(|| from_smiles(DEFAULT_SMILES));
    let toast = use_context::<ToastManager>();
    let cosmolkit_version = cosmolkit::version();

    rsx! {
        Seo {
            title: "InChI Converter — InChI, InChIKey & Molecular Structure | COSMolKit",
            description: "Convert SMILES to standard InChI and InChIKey or parse InChI back to molecular structures using COSMolKit's pure Rust InChI implementation directly in your browser.",
            canonical: "https://tools.cosmol.org/tools/inchi",
        }
        div { class: "min-h-screen uu-backdrop m-0 pt-[74px]",
            main { class: "mx-auto w-full max-w-6xl px-0 py-5 font-sans text-[#e8edf5] max-[800px]:px-3.5 max-[800px]:pb-[30px]",
                section { class: "w-full",
                    div { class: "mb-6 flex items-end justify-between gap-8 max-[800px]:flex-col max-[800px]:items-start max-[800px]:gap-4",
                        div {
                            Link { class: "text-[13px] font-semibold text-[#7ab5ff] no-underline hover:text-[#b4d6ff]", to: crate::route::Route::ToolDirectory {}, "Back to tools" }
                            h1 { class: "mb-1.5 mt-2.5 text-[32px] leading-[1.2] font-bold text-slate-50 max-[800px]:text-[27px]", "InChI workspace" }
                            p { class: "m-0 max-w-[680px] text-[15px] leading-[1.6] text-[#9caabd]", "Generate standard InChI identifiers or recover molecular structures locally in WebAssembly." }
                        }
                        a {
                            class: "inline-flex shrink-0 items-center gap-2 rounded-md border border-[#23344a] bg-[#0c1828] px-[11px] py-2 text-xs font-semibold text-[#b8c5d6] no-underline hover:border-[#438ee9] hover:text-white",
                            href: "https://crates.io/crates/cosmolkit", target: "_blank", rel: "noreferrer",
                            "COSMolKit {cosmolkit_version} / Rust / WASM"
                            MdiIcon { size: 14, path: MDI_OPEN_IN_NEW }
                        }
                    }

                    div { class: "grid min-h-[660px] grid-cols-[350px_minmax(0,1fr)] overflow-hidden rounded-lg border border-[#213147] bg-[#0c1727] shadow-[0_20px_55px_rgba(0,0,0,0.24)] max-[820px]:grid-cols-1",
                        aside { class: "border-r border-[#213147] bg-[#0a1524] p-6 max-[820px]:border-r-0 max-[820px]:border-b max-[480px]:p-[18px]",
                            div { class: "mb-6 grid grid-cols-2 rounded-md border border-[#2a3b52] bg-[#07111f] p-1",
                                button {
                                    r#type: "button",
                                    class: if mode() == InchiMode::FromSmiles { "h-9 cursor-pointer rounded-[4px] bg-[#267de8] px-2 text-xs font-bold text-white" } else { "h-9 cursor-pointer rounded-[4px] px-2 text-xs font-semibold text-[#91a1b5] hover:text-white" },
                                    onclick: move |_| {
                                        mode.set(InchiMode::FromSmiles);
                                        input.set(DEFAULT_SMILES.to_string());
                                        result.set(from_smiles(DEFAULT_SMILES));
                                    },
                                    "SMILES -> InChI"
                                }
                                button {
                                    r#type: "button",
                                    class: if mode() == InchiMode::ToStructure { "h-9 cursor-pointer rounded-[4px] bg-[#267de8] px-2 text-xs font-bold text-white" } else { "h-9 cursor-pointer rounded-[4px] px-2 text-xs font-semibold text-[#91a1b5] hover:text-white" },
                                    onclick: move |_| {
                                        mode.set(InchiMode::ToStructure);
                                        input.set(DEFAULT_INCHI.to_string());
                                        result.set(to_structure(DEFAULT_INCHI));
                                    },
                                    "InChI -> structure"
                                }
                            }

                            div { class: "mb-6",
                                div { class: "flex items-center justify-between",
                                    label { class: "mb-[9px] block text-[13px] font-bold text-[#dce5f0]", r#for: "inchi-input", if mode() == InchiMode::FromSmiles { "SMILES" } else { "InChI" } }
                                    span { class: "text-[11px] text-[#6f8095]", "{input().len()} / {MAX_INPUT_LENGTH}" }
                                }
                                textarea {
                                    id: "inchi-input", maxlength: MAX_INPUT_LENGTH, spellcheck: false,
                                    class: "block min-h-[164px] w-full resize-y rounded-md border border-[#2a3b52] bg-[#07111f] px-3.5 py-[13px] font-mono text-[13px] leading-[1.65] text-[#eaf1fa] outline-none hover:border-[#438ee9] focus:border-[#438ee9] focus:ring-[3px] focus:ring-[#3082ff1f]",
                                    value: "{input}",
                                    oninput: move |event| input.set(event.value()),
                                }
                            }

                            div { class: "mb-6",
                                label { class: "mb-[9px] block text-[13px] font-bold text-[#dce5f0]", "Examples" }
                                div { class: "flex flex-wrap gap-[7px]",
                                    for (name, smiles_value, inchi_value) in EXAMPLES {
                                        button {
                                            r#type: "button",
                                            class: "cursor-pointer rounded-[5px] border border-[#2b3d54] bg-[#101e30] px-[9px] py-1.5 text-xs text-[#adbbcc] hover:border-[#438ee9] hover:text-[#eaf3ff]",
                                            onclick: move |_| {
                                                let value = if mode() == InchiMode::FromSmiles { smiles_value } else { inchi_value };
                                                input.set(value.to_string());
                                                result.set(convert(value, mode()));
                                            },
                                            "{name}"
                                        }
                                    }
                                }
                            }

                            button {
                                r#type: "button",
                                class: "h-[42px] w-full cursor-pointer rounded-md border border-[#398cf6] bg-[#267de8] text-[13px] font-bold text-white shadow-[0_8px_24px_rgba(38,125,232,0.2)] hover:bg-[#358bf3] active:translate-y-px",
                                onclick: move |_| result.set(convert(&input(), mode())),
                                if mode() == InchiMode::FromSmiles { "Generate identifiers" } else { "Parse InChI" }
                            }
                            p { class: "mt-[13px] mb-0 text-center text-[11px] text-[#718299]", "Official InChI source port, executed on this device." }
                        }

                        section { class: "grid min-w-0 grid-rows-[auto_auto_1fr] bg-[#111c2c]",
                            div { class: "flex min-h-16 items-center justify-between gap-5 border-b border-[#213147] px-[18px] py-3 max-[560px]:flex-col max-[560px]:items-start",
                                div {
                                    h2 { class: "mt-0 mb-[3px] text-sm font-bold text-[#eef4fb]", "Identifiers" }
                                    if let Ok(value) = &*result.read() {
                                        span { class: "text-[11px] text-[#718299]", "{value.atom_count} atoms  /  {value.bond_count} bonds" }
                                    }
                                }
                                if let Ok(value) = &*result.read() {
                                    div { class: "flex gap-2 max-[480px]:w-full",
                                        button {
                                            r#type: "button", class: "h-[34px] cursor-pointer rounded-[5px] border border-[#30435b] bg-[#0c1828] px-[11px] text-xs font-semibold text-[#c5d1df] hover:border-[#438ee9] hover:text-white max-[480px]:flex-1",
                                            onclick: { let text = value.inchi.clone(); move |_| copy_text(text.clone(), "InChI copied.", toast) },
                                            "Copy InChI"
                                        }
                                        a { class: "inline-flex h-[34px] items-center justify-center rounded-[5px] border border-[#30435b] bg-[#0c1828] px-[11px] text-xs font-semibold text-[#c5d1df] no-underline hover:border-[#438ee9] hover:text-white max-[480px]:flex-1", href: value.download_url.clone(), download: "molecule.inchi", "Download" }
                                    }
                                }
                            }

                            match &*result.read() {
                                Ok(value) => rsx! {
                                    div { class: "grid gap-3 border-b border-[#213147] bg-[#0d1929] p-[18px]",
                                        div { class: "grid grid-cols-[92px_minmax(0,1fr)_auto] items-start gap-3 max-[560px]:grid-cols-1",
                                            span { class: "pt-2 text-[11px] font-bold text-[#718299]", "INCHI" }
                                            code { class: "min-w-0 break-all rounded-[5px] border border-[#26384f] bg-[#07111f] px-3 py-2 font-mono text-xs leading-5 text-[#cfe2fa]", "{value.inchi}" }
                                            button { r#type: "button", class: "h-9 cursor-pointer rounded-[5px] border border-[#30435b] bg-[#0c1828] px-3 text-xs font-semibold text-[#c5d1df] hover:border-[#438ee9] hover:text-white", onclick: { let text = value.inchi.clone(); move |_| copy_text(text.clone(), "InChI copied.", toast) }, "Copy" }
                                        }
                                        div { class: "grid grid-cols-[92px_minmax(0,1fr)_auto] items-start gap-3 max-[560px]:grid-cols-1",
                                            span { class: "pt-2 text-[11px] font-bold text-[#718299]", "INCHIKEY" }
                                            code { class: "min-w-0 break-all rounded-[5px] border border-[#26384f] bg-[#07111f] px-3 py-2 font-mono text-xs leading-5 text-[#a7ead3]", "{value.inchi_key}" }
                                            button { r#type: "button", class: "h-9 cursor-pointer rounded-[5px] border border-[#30435b] bg-[#0c1828] px-3 text-xs font-semibold text-[#c5d1df] hover:border-[#438ee9] hover:text-white", onclick: { let text = value.inchi_key.clone(); move |_| copy_text(text.clone(), "InChIKey copied.", toast) }, "Copy" }
                                        }
                                        div { class: "grid grid-cols-[92px_minmax(0,1fr)_auto] items-start gap-3 max-[560px]:grid-cols-1",
                                            span { class: "pt-2 text-[11px] font-bold text-[#718299]", "SMILES" }
                                            code { class: "min-w-0 break-all rounded-[5px] border border-[#26384f] bg-[#07111f] px-3 py-2 font-mono text-xs leading-5 text-[#dbe5f2]", "{value.smiles}" }
                                            button { r#type: "button", class: "h-9 cursor-pointer rounded-[5px] border border-[#30435b] bg-[#0c1828] px-3 text-xs font-semibold text-[#c5d1df] hover:border-[#438ee9] hover:text-white", onclick: { let text = value.smiles.clone(); move |_| copy_text(text.clone(), "SMILES copied.", toast) }, "Copy" }
                                        }
                                    }
                                    div { class: "grid min-h-[360px] place-items-center bg-[#e9eef4] p-5 max-[480px]:p-3",
                                        div { class: "grid h-full min-h-[320px] w-full place-items-center overflow-hidden border border-[#d5dde7] bg-white",
                                            img { class: "block h-full max-h-full w-full max-w-full object-contain", src: value.svg_url.clone(), alt: "Molecular structure represented by the InChI" }
                                        }
                                    }
                                },
                                Err(error) => rsx! {
                                    div { class: "row-span-2 grid min-h-[520px] place-items-center bg-[#e9eef4] p-6 text-center",
                                        div { class: "max-w-[480px]",
                                            div { class: "mx-auto mb-3.5 grid h-[42px] w-[42px] place-items-center rounded-full border border-[#e6a9a9] bg-[#fff5f5] text-lg font-extrabold text-[#c83e3e]", "!" }
                                            h3 { class: "mb-[7px] mt-0 text-base font-bold text-[#172234]", "Unable to convert" }
                                            p { class: "m-0 text-[13px] leading-[1.55] text-[#68778a]", "{error}" }
                                        }
                                    }
                                },
                            }
                        }
                    }
                }

                section { class: "mt-10 border-t border-[#213147] pt-8",
                    span { class: "text-xs font-bold text-[#4b96ff]", "SUPPORTED INCHI WORKFLOWS" }
                    h2 { class: "mb-2 mt-2 text-xl font-bold text-slate-50", "Identifiers and molecular structures" }
                    p { class: "m-0 max-w-[860px] text-sm leading-6 text-[#9caabd]",
                        "This tool supports SMILES to standard InChI, SMILES to InChIKey, InChI to a molecular structure, and InChI to InChIKey. Processing stays on this device and uses COSMolKit's pure-Rust InChI engine."
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
    fn methane_has_official_standard_identifiers() {
        let result = from_smiles("C").expect("methane identifiers");
        assert_eq!(result.inchi, "InChI=1S/CH4/h1H4");
        assert_eq!(result.inchi_key, "VNWKTOKETHGBQD-UHFFFAOYSA-N");
    }

    #[test]
    fn inchi_round_trip_recovers_a_graph() {
        let result = to_structure("InChI=1S/C2H6O/c1-2-3/h3H,2H2,1H3").expect("ethanol structure");
        assert_eq!(result.atom_count, 3);
        assert_eq!(result.bond_count, 2);
        assert_eq!(result.inchi_key, "LFQSCWFLJHTTHZ-UHFFFAOYSA-N");
    }

    #[test]
    fn rejects_non_inchi_text_before_calling_the_engine() {
        assert!(
            to_structure("not an identifier")
                .unwrap_err()
                .contains("InChI=")
        );
    }
}
