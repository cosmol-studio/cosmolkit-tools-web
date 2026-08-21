use cosmolkit::{Molecule, SmilesWriteParams};
use dioxus::prelude::*;

use crate::component::{MdiIcon, Seo, ToastManager, icon::MDI_OPEN_IN_NEW};

const DEFAULT_SMILES: &str = "OCC";
const MAX_INPUT_LENGTH: usize = 16_384;
const EXAMPLES: [(&str, &str); 4] = [
    ("Ethanol", "OCC"),
    ("L-alanine", "N[C@@H](C)C(=O)O"),
    ("Benzene", "c1ccccc1"),
    ("Acetate", "CC(=O)[O-]"),
];
const PYTHON_CODE: &str = r#"from cosmolkit import Molecule

mol = Molecule.from_smiles("OCC")
result = {
    "canonical_smiles": mol.to_smiles(
        isomeric_smiles=False, canonical=True
    ),
    "isomeric_smiles": mol.to_smiles(
        isomeric_smiles=True, canonical=True
    ),
    "kekulized_smiles": mol.to_smiles(canonical=True, kekule=True),
    "hydrogen_count": sum(
        a.atomic_num() == 1 for a in mol.with_hydrogens().atoms()
    ),
    "formal_charge": sum(a.formal_charge() for a in mol.atoms()),
}
print(result)"#;

#[derive(Clone, Debug, PartialEq)]
struct CanonicalResult {
    canonical_smiles: String,
    isomeric_smiles: String,
    kekulized_smiles: String,
    hydrogen_count: usize,
    formal_charge: i32,
    atom_count: usize,
    svg_url: String,
}

fn data_url(content: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut encoded = String::with_capacity(content.len() * 2);
    encoded.push_str("data:image/svg+xml;charset=utf-8,");
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

fn canonicalize(input: &str) -> Result<CanonicalResult, String> {
    let smiles = input.trim();
    if smiles.is_empty() {
        return Err("Enter a SMILES string to canonicalize.".to_string());
    }
    if smiles.len() > MAX_INPUT_LENGTH {
        return Err(format!(
            "SMILES input is too long (maximum {MAX_INPUT_LENGTH} characters)."
        ));
    }

    let molecule = Molecule::from_smiles(smiles)
        .map_err(|error| format!("Could not parse this SMILES: {error}"))?;
    let canonical_smiles = molecule
        .to_smiles_with_params(&SmilesWriteParams {
            do_isomeric_smiles: false,
            ..SmilesWriteParams::default()
        })
        .map_err(|error| format!("Could not write canonical SMILES: {error}"))?;
    let isomeric_smiles = molecule
        .to_smiles_with_params(&SmilesWriteParams::default())
        .map_err(|error| format!("Could not write isomeric SMILES: {error}"))?;
    let kekulized_smiles = molecule
        .to_smiles_with_params(&SmilesWriteParams {
            do_kekule: true,
            ..SmilesWriteParams::default()
        })
        .map_err(|error| format!("Could not write kekulized SMILES: {error}"))?;
    let hydrogen_count = molecule
        .with_hydrogens()
        .map_err(|error| format!("Could not count hydrogens: {error}"))?
        .atoms()
        .iter()
        .filter(|atom| atom.atomic_number() == 1)
        .count();
    let svg = molecule
        .to_svg(640, 420)
        .map_err(|error| format!("Could not draw this molecule: {error}"))?;

    Ok(CanonicalResult {
        canonical_smiles,
        isomeric_smiles,
        kekulized_smiles,
        hydrogen_count,
        formal_charge: molecule
            .atoms()
            .iter()
            .map(|atom| i32::from(atom.formal_charge()))
            .sum(),
        atom_count: molecule.num_atoms(),
        svg_url: data_url(&svg),
    })
}

fn format_charge(charge: i32) -> String {
    match charge.cmp(&0) {
        std::cmp::Ordering::Greater => format!("+{charge}"),
        _ => charge.to_string(),
    }
}

#[cfg(target_arch = "wasm32")]
fn copy_text(text: String, mut toast: ToastManager) {
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
            Ok(()) => toast.success("SMILES copied."),
            Err(message) => toast.error(message),
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn copy_text(_text: String, mut toast: ToastManager) {
    toast.info("Clipboard export is available in the web build.");
}

#[component]
pub fn SmilesCanonicalizer() -> Element {
    let mut input = use_signal(|| DEFAULT_SMILES.to_string());
    let mut result = use_signal(|| canonicalize(DEFAULT_SMILES));
    let toast = use_context::<ToastManager>();
    let cosmolkit_version = cosmolkit::version();

    rsx! {
        Seo {
            title: "SMILES Canonicalizer — Canonical & Isomeric SMILES | COSMolKit",
            description: "Canonicalize SMILES locally in your browser. Generate canonical, isomeric and kekulized SMILES and inspect hydrogen count and formal charge with COSMolKit.",
            canonical: "https://tools.cosmol.org/smiles-canonicalizer",
        }
        div { class: "min-h-screen uu-backdrop m-0 pt-[74px]",
            main { class: "mx-auto w-full max-w-6xl px-0 py-5 font-sans text-[#e8edf5] max-[800px]:px-3.5 max-[800px]:pb-[30px]",
                section { class: "w-full",
                    div { class: "mb-6 flex items-end justify-between gap-8 max-[800px]:flex-col max-[800px]:items-start max-[800px]:gap-4",
                        div {
                            Link { class: "text-[13px] font-semibold text-[#7ab5ff] no-underline hover:text-[#b4d6ff]", to: crate::route::Route::ToolDirectory {}, "Back to tools" }
                            h1 { class: "mb-1.5 mt-2.5 text-[32px] leading-[1.2] font-bold text-slate-50 max-[800px]:text-[27px]", "SMILES canonicalizer" }
                            p { class: "m-0 max-w-[700px] text-[15px] leading-[1.6] text-[#9caabd]", "Normalize SMILES serialization and inspect stereochemistry, aromatic bond form, hydrogens, and charge." }
                        }
                        a {
                            class: "inline-flex shrink-0 items-center gap-2 rounded-md border border-[#23344a] bg-[#0c1828] px-[11px] py-2 text-xs font-semibold text-[#b8c5d6] no-underline hover:border-[#438ee9] hover:text-white",
                            href: "https://crates.io/crates/cosmolkit", target: "_blank", rel: "noreferrer",
                            "COSMolKit {cosmolkit_version} / Rust / WASM"
                            MdiIcon { size: 14, path: MDI_OPEN_IN_NEW }
                        }
                    }

                    div { class: "grid min-h-[650px] grid-cols-[350px_minmax(0,1fr)] overflow-hidden rounded-lg border border-[#213147] bg-[#0c1727] shadow-[0_20px_55px_rgba(0,0,0,0.24)] max-[820px]:grid-cols-1",
                        aside { class: "border-r border-[#213147] bg-[#0a1524] p-6 max-[820px]:border-r-0 max-[820px]:border-b max-[480px]:p-[18px]",
                            div { class: "mb-6",
                                div { class: "flex items-center justify-between",
                                    label { class: "mb-[9px] block text-[13px] font-bold text-[#dce5f0]", r#for: "canonical-smiles", "SMILES" }
                                    span { class: "text-[11px] text-[#6f8095]", "{input().len()} / {MAX_INPUT_LENGTH}" }
                                }
                                textarea {
                                    id: "canonical-smiles", maxlength: MAX_INPUT_LENGTH, spellcheck: false,
                                    class: "block min-h-[164px] w-full resize-y rounded-md border border-[#2a3b52] bg-[#07111f] px-3.5 py-[13px] font-mono text-[13px] leading-[1.65] text-[#eaf1fa] outline-none hover:border-[#438ee9] focus:border-[#438ee9] focus:ring-[3px] focus:ring-[#3082ff1f]",
                                    value: "{input}",
                                    oninput: move |event| {
                                        let value = event.value();
                                        result.set(canonicalize(&value));
                                        input.set(value);
                                    },
                                }
                            }
                            div { class: "mb-7",
                                label { class: "mb-[9px] block text-[13px] font-bold text-[#dce5f0]", "Examples" }
                                div { class: "flex flex-wrap gap-[7px]",
                                    for (name, value) in EXAMPLES {
                                        button {
                                            r#type: "button",
                                            class: "cursor-pointer rounded-[5px] border border-[#2b3d54] bg-[#101e30] px-[9px] py-1.5 text-xs text-[#adbbcc] hover:border-[#438ee9] hover:text-[#eaf3ff]",
                                            onclick: move |_| {
                                                input.set(value.to_string());
                                                result.set(canonicalize(value));
                                            },
                                            "{name}"
                                        }
                                    }
                                }
                            }
                            div { class: "border-t border-[#213147] pt-5",
                                span { class: "text-[11px] font-bold text-[#718299]", "SCOPE" }
                                p { class: "mb-0 mt-2 text-xs leading-5 text-[#718299]", "This tool canonicalizes SMILES serialization. It preserves molecular components and formal charges; it does not remove salts or neutralize the structure." }
                            }
                        }

                        section { class: "min-w-0 bg-[#111c2c]",
                            match &*result.read() {
                                Ok(value) => rsx! {
                                    div { class: "flex min-h-16 items-center justify-between gap-5 border-b border-[#213147] px-6 py-3 max-[480px]:px-[18px]",
                                        div {
                                            h2 { class: "mb-1 mt-0 text-sm font-bold text-[#eef4fb]", "Normalized outputs" }
                                            span { class: "text-[11px] text-[#718299]", "{value.atom_count} graph atoms / {value.hydrogen_count} total hydrogens" }
                                        }
                                    }
                                    div { class: "grid grid-cols-[minmax(0,1fr)_230px] max-[700px]:grid-cols-1",
                                        div { class: "min-w-0 divide-y divide-[#213147]",
                                            SmilesOutput { label: "Canonical SMILES", value: value.canonical_smiles.clone() }
                                            SmilesOutput { label: "Isomeric SMILES", value: value.isomeric_smiles.clone() }
                                            SmilesOutput { label: "Kekulized SMILES", value: value.kekulized_smiles.clone() }
                                            div { class: "grid grid-cols-2 gap-px bg-[#213147]",
                                                div { class: "bg-[#111c2c] px-6 py-5 max-[480px]:px-[18px]",
                                                    span { class: "block text-[11px] font-bold text-[#718299]", "H COUNT" }
                                                    strong { class: "mt-1.5 block text-xl text-[#f1f6fc]", "{value.hydrogen_count}" }
                                                }
                                                div { class: "bg-[#111c2c] px-6 py-5 max-[480px]:px-[18px]",
                                                    span { class: "block text-[11px] font-bold text-[#718299]", "FORMAL CHARGE" }
                                                    strong { class: "mt-1.5 block text-xl text-[#f1f6fc]", "{format_charge(value.formal_charge)}" }
                                                }
                                            }
                                        }
                                        div { class: "grid min-h-[430px] place-items-center border-l border-[#213147] bg-[#e9eef4] p-4 max-[700px]:min-h-[300px] max-[700px]:border-l-0 max-[700px]:border-t",
                                            img { class: "h-full max-h-[390px] w-full object-contain", src: value.svg_url.clone(), alt: "Canonicalized molecular structure" }
                                        }
                                    }
                                },
                                Err(error) => rsx! {
                                    div { class: "grid min-h-[650px] place-items-center p-8 text-center",
                                        div { class: "max-w-[460px]",
                                            div { class: "mx-auto mb-3.5 grid h-[42px] w-[42px] place-items-center rounded-full border border-[#6d3f49] bg-[#2b1720] text-lg font-extrabold text-[#f09aa9]", "!" }
                                            h2 { class: "mb-2 mt-0 text-base font-bold text-[#eef4fb]", "Unable to canonicalize SMILES" }
                                            p { class: "m-0 text-[13px] leading-6 text-[#9caabd]", "{error}" }
                                        }
                                    }
                                },
                            }
                        }
                    }
                }

                section { class: "mt-10 border-t border-[#213147] pt-8",
                    span { class: "text-xs font-bold text-[#4b96ff]", "CANONICAL SMILES" }
                    h2 { class: "mb-2 mt-2 text-xl font-bold text-slate-50", "Convert equivalent SMILES to a consistent representation" }
                    p { class: "m-0 max-w-[860px] text-sm leading-6 text-[#9caabd]", "COSMolKit parses the molecular graph and writes canonical SMILES, stereochemistry-aware isomeric SMILES, and an explicit kekulized aromatic representation. Hydrogen count and total formal charge are derived from the same graph locally in WebAssembly." }
                }

                section { class: "mt-10 border-t border-[#213147] pt-8",
                    div { class: "flex items-start justify-between gap-6 max-[800px]:flex-col",
                        div {
                            span { class: "text-xs font-bold text-[#4b96ff]", "PYTHON BACKEND" }
                            h2 { class: "mb-1.5 mt-2 text-xl font-bold text-slate-50", "Canonicalize SMILES in Python" }
                            p { class: "m-0 max-w-[680px] text-sm leading-6 text-[#9caabd]", "Use the COSMolKit Python bindings to generate the same canonical, isomeric, and kekulized outputs." }
                        }
                        a { class: "inline-flex shrink-0 items-center gap-2 rounded-md border border-[#2a3b52] bg-[#0c1828] px-3 py-2 text-xs font-semibold text-[#b8c5d6] no-underline hover:border-[#438ee9] hover:text-white", href: "https://pypi.org/project/cosmolkit/{cosmolkit_version}/", target: "_blank", rel: "noreferrer", "COSMolKit Python {cosmolkit_version}" MdiIcon { size: 14, path: MDI_OPEN_IN_NEW } }
                    }
                    div { class: "mt-6 min-w-0 overflow-hidden rounded-lg border border-[#213147] bg-[#081321]",
                        div { class: "flex min-h-11 items-center justify-between border-b border-[#213147] px-4",
                            span { class: "text-xs font-semibold text-[#9caabd]", "canonicalize_smiles.py" }
                            button { r#type: "button", class: "cursor-pointer rounded-[5px] border border-[#30435b] bg-[#0c1828] px-2.5 py-1.5 text-[11px] font-semibold text-[#c5d1df] hover:border-[#438ee9] hover:text-white", onclick: move |_| copy_text(PYTHON_CODE.to_string(), toast), "Copy" }
                        }
                        pre { class: "m-0 max-h-[430px] overflow-auto p-5 text-[12px] leading-5 text-[#cbd8e8]", code { "{PYTHON_CODE}" } }
                    }
                }
            }
        }
    }
}

#[component]
fn SmilesOutput(label: String, value: String) -> Element {
    let toast = use_context::<ToastManager>();
    rsx! {
        div { class: "px-6 py-5 max-[480px]:px-[18px]",
            div { class: "mb-2 flex items-center justify-between gap-4",
                span { class: "text-[11px] font-bold text-[#718299]", "{label}" }
                button {
                    r#type: "button", title: "Copy {label}",
                    class: "cursor-pointer rounded-[5px] border border-[#30435b] bg-[#0c1828] px-2.5 py-1 text-[11px] font-semibold text-[#c5d1df] hover:border-[#438ee9] hover:text-white",
                    onclick: { let text = value.clone(); move |_| copy_text(text.clone(), toast) },
                    "Copy"
                }
            }
            code { class: "block break-all font-mono text-[13px] leading-6 text-[#e2ecf8]", "{value}" }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ethanol_is_canonicalized_from_occ_to_cco() {
        let result = canonicalize("OCC").expect("canonical ethanol");
        assert_eq!(result.canonical_smiles, "CCO");
        assert_eq!(result.isomeric_smiles, "CCO");
        assert_eq!(result.kekulized_smiles, "CCO");
        assert_eq!(result.hydrogen_count, 6);
        assert_eq!(result.formal_charge, 0);
    }

    #[test]
    fn canonicalizer_preserves_stereochemistry_and_charge() {
        let stereo = canonicalize("N[C@@H](C)C(=O)O").expect("alanine");
        assert!(stereo.isomeric_smiles.contains('@'));
        let acetate = canonicalize("CC(=O)[O-]").expect("acetate");
        assert_eq!(acetate.formal_charge, -1);
    }

    #[test]
    fn benzene_has_a_kekulized_output() {
        let result = canonicalize("c1ccccc1").expect("benzene");
        assert!(result.canonical_smiles.contains('c'));
        assert!(result.kekulized_smiles.contains('='));
    }
}
