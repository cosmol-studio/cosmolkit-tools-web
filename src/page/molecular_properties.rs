use cosmolkit::{
    Molecule, NumRotatableBondsOptions, calc_crippen_descriptors, calc_exact_mol_wt,
    calc_mol_formula, calc_mol_wt, calc_num_hba, calc_num_hbd, calc_num_rotatable_bonds, calc_tpsa,
};
use dioxus::prelude::*;

use crate::component::{MdiIcon, Seo, ToastManager, icon::MDI_OPEN_IN_NEW};

const DEFAULT_SMILES: &str = "CC(=O)Oc1ccccc1C(=O)O";
const MAX_INPUT_LENGTH: usize = 16_384;
const EXAMPLES: [(&str, &str); 4] = [
    ("Aspirin", DEFAULT_SMILES),
    ("Caffeine", "Cn1c(=O)c2c(ncn2C)n(C)c1=O"),
    ("Ethanol", "CCO"),
    ("Ibuprofen", "CC(C)Cc1ccc(cc1)[C@@H](C)C(=O)O"),
];
const PYTHON_CODE: &str = r#"from cosmolkit import (
    Molecule, calc_crippen_descriptors, calc_exact_mol_wt,
    calc_mol_formula, calc_mol_wt, calc_num_hba, calc_num_hbd,
    calc_num_rotatable_bonds, calc_tpsa,
)

mol = Molecule.from_smiles("CCO")
logp, _ = calc_crippen_descriptors(mol)

properties = {
    "formula": calc_mol_formula(mol),
    "molecular_weight": calc_mol_wt(mol),
    "exact_mass": calc_exact_mol_wt(mol),
    "heavy_atoms": sum(a.atomic_num() != 1 for a in mol.atoms()),
    "hbd": calc_num_hbd(mol),
    "hba": calc_num_hba(mol),
    "tpsa": calc_tpsa(mol),
    "rotatable_bonds": calc_num_rotatable_bonds(mol, mode="strict"),
    "logp": logp,
    "formal_charge": sum(a.formal_charge() for a in mol.atoms()),
}
print(properties)"#;

#[derive(Clone, Debug, PartialEq)]
struct PropertyResult {
    formula: String,
    molecular_weight: f64,
    exact_mass: f64,
    heavy_atoms: usize,
    hbd: u32,
    hba: u32,
    tpsa: f64,
    rotatable_bonds: u32,
    logp: f64,
    formal_charge: i32,
    canonical_smiles: String,
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

fn calculate(input: &str) -> Result<PropertyResult, String> {
    let smiles = input.trim();
    if smiles.is_empty() {
        return Err("Enter a SMILES string to calculate molecular properties.".to_string());
    }
    if smiles.len() > MAX_INPUT_LENGTH {
        return Err(format!(
            "SMILES input is too long (maximum {MAX_INPUT_LENGTH} characters)."
        ));
    }

    let molecule = Molecule::from_smiles(smiles)
        .map_err(|error| format!("Could not parse this SMILES: {error}"))?;
    let descriptor_error = |name: &str, error| format!("Could not calculate {name}: {error}");
    let crippen = calc_crippen_descriptors(&molecule, true, false)
        .map_err(|error| descriptor_error("logP", error))?;
    let svg = molecule
        .to_svg(640, 420)
        .map_err(|error| format!("Could not draw this molecule: {error}"))?;

    Ok(PropertyResult {
        formula: calc_mol_formula(&molecule, false, true)
            .map_err(|error| descriptor_error("molecular formula", error))?,
        molecular_weight: calc_mol_wt(&molecule, false)
            .map_err(|error| descriptor_error("molecular weight", error))?,
        exact_mass: calc_exact_mol_wt(&molecule, false)
            .map_err(|error| descriptor_error("exact mass", error))?,
        heavy_atoms: molecule
            .atoms()
            .iter()
            .filter(|atom| atom.atomic_number() != 1)
            .count(),
        hbd: calc_num_hbd(&molecule).map_err(|error| descriptor_error("HBD", error))?,
        hba: calc_num_hba(&molecule).map_err(|error| descriptor_error("HBA", error))?,
        tpsa: calc_tpsa(&molecule, false, false)
            .map_err(|error| descriptor_error("TPSA", error))?,
        rotatable_bonds: calc_num_rotatable_bonds(&molecule, NumRotatableBondsOptions::Strict)
            .map_err(|error| descriptor_error("rotatable bonds", error))?,
        logp: crippen.logp,
        formal_charge: molecule
            .atoms()
            .iter()
            .map(|atom| i32::from(atom.formal_charge()))
            .sum(),
        canonical_smiles: molecule
            .to_smiles(true)
            .map_err(|error| format!("Could not write canonical SMILES: {error}"))?,
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
fn copy_python(text: String, mut toast: ToastManager) {
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
            Ok(()) => toast.success("Python example copied."),
            Err(message) => toast.error(message),
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn copy_python(_text: String, mut toast: ToastManager) {
    toast.info("Clipboard export is available in the web build.");
}

#[component]
pub fn MolecularProperties() -> Element {
    let mut input = use_signal(|| DEFAULT_SMILES.to_string());
    let mut result = use_signal(|| calculate(DEFAULT_SMILES));
    let toast = use_context::<ToastManager>();
    let cosmolkit_version = cosmolkit::version();

    rsx! {
        Seo {
            title: "Molecular Properties Calculator — MW, TPSA, logP | COSMolKit",
            description: "Calculate molecular weight, exact mass, TPSA, logP, HBD, HBA, and more online with COSMolKit's Rust cheminformatics core, entirely in your browser.",
            canonical: "https://tools.cosmol.org/molecular-properties",
        }
        div { class: "min-h-screen uu-backdrop m-0 pt-[74px]",
            main { class: "mx-auto w-full max-w-6xl px-0 py-5 font-sans text-[#e8edf5] max-[800px]:px-3.5 max-[800px]:pb-[30px]",
                section { class: "w-full",
                    div { class: "mb-6 flex items-end justify-between gap-8 max-[800px]:flex-col max-[800px]:items-start max-[800px]:gap-4",
                        div {
                            Link { class: "text-[13px] font-semibold text-[#7ab5ff] no-underline hover:text-[#b4d6ff]", to: crate::route::Route::ToolDirectory {}, "Back to tools" }
                            h1 { class: "mb-1.5 mt-2.5 text-[32px] leading-[1.2] font-bold text-slate-50 max-[800px]:text-[27px]", "Molecular properties calculator" }
                            p { class: "m-0 max-w-[700px] text-[15px] leading-[1.6] text-[#9caabd]", "Calculate commonly used molecular descriptors from SMILES locally in your browser." }
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
                                    label { class: "mb-[9px] block text-[13px] font-bold text-[#dce5f0]", r#for: "properties-smiles", "SMILES" }
                                    span { class: "text-[11px] text-[#6f8095]", "{input().len()} / {MAX_INPUT_LENGTH}" }
                                }
                                textarea {
                                    id: "properties-smiles", maxlength: MAX_INPUT_LENGTH, spellcheck: false,
                                    class: "block min-h-[164px] w-full resize-y rounded-md border border-[#2a3b52] bg-[#07111f] px-3.5 py-[13px] font-mono text-[13px] leading-[1.65] text-[#eaf1fa] outline-none hover:border-[#438ee9] focus:border-[#438ee9] focus:ring-[3px] focus:ring-[#3082ff1f]",
                                    value: "{input}",
                                    oninput: move |event| {
                                        let value = event.value();
                                        result.set(calculate(&value));
                                        input.set(value);
                                    },
                                }
                            }
                            div {
                                label { class: "mb-[9px] block text-[13px] font-bold text-[#dce5f0]", "Examples" }
                                div { class: "flex flex-wrap gap-[7px]",
                                    for (name, value) in EXAMPLES {
                                        button {
                                            r#type: "button",
                                            class: "cursor-pointer rounded-[5px] border border-[#2b3d54] bg-[#101e30] px-[9px] py-1.5 text-xs text-[#adbbcc] hover:border-[#438ee9] hover:text-[#eaf3ff]",
                                            onclick: move |_| {
                                                input.set(value.to_string());
                                                result.set(calculate(value));
                                            },
                                            "{name}"
                                        }
                                    }
                                }
                            }
                            div { class: "mt-7 border-t border-[#213147] pt-5",
                                p { class: "m-0 text-xs leading-5 text-[#718299]", "Descriptors are calculated on this device from the parsed COSMolKit molecular graph." }
                            }
                        }

                        section { class: "min-w-0 bg-[#111c2c]",
                            match &*result.read() {
                                Ok(value) => rsx! {
                                    div { class: "border-b border-[#213147] px-6 py-5 max-[480px]:px-[18px]",
                                        div { class: "flex items-start justify-between gap-5 max-[600px]:flex-col",
                                            div { class: "min-w-0",
                                                span { class: "text-[11px] font-bold text-[#718299]", "CANONICAL STRUCTURE" }
                                                code { class: "mt-2 block break-all font-mono text-[13px] leading-5 text-[#dce8f7]", "{value.canonical_smiles}" }
                                            }
                                            div { class: "h-[110px] w-[180px] shrink-0 overflow-hidden rounded-md border border-[#d7dee7] bg-white p-2 max-[600px]:w-full",
                                                img { class: "h-full w-full object-contain", src: value.svg_url.clone(), alt: "Molecular structure preview" }
                                            }
                                        }
                                    }
                                    div { class: "grid grid-cols-2 gap-px bg-[#213147] max-[560px]:grid-cols-1",
                                        PropertyCell { label: "Formula", value: value.formula.clone(), unit: "" }
                                        PropertyCell { label: "Molecular weight", value: format!("{:.3}", value.molecular_weight), unit: "g/mol" }
                                        PropertyCell { label: "Exact mass", value: format!("{:.5}", value.exact_mass), unit: "Da" }
                                        PropertyCell { label: "Heavy atoms", value: value.heavy_atoms.to_string(), unit: "atoms" }
                                        PropertyCell { label: "H-bond donors", value: value.hbd.to_string(), unit: "HBD" }
                                        PropertyCell { label: "H-bond acceptors", value: value.hba.to_string(), unit: "HBA" }
                                        PropertyCell { label: "Topological polar surface area", value: format!("{:.2}", value.tpsa), unit: "A²" }
                                        PropertyCell { label: "Rotatable bonds", value: value.rotatable_bonds.to_string(), unit: "strict" }
                                        PropertyCell { label: "Crippen logP", value: format!("{:.3}", value.logp), unit: "logP" }
                                        PropertyCell { label: "Formal charge", value: format_charge(value.formal_charge), unit: "e" }
                                    }
                                },
                                Err(error) => rsx! {
                                    div { class: "grid min-h-[650px] place-items-center p-8 text-center",
                                        div { class: "max-w-[460px]",
                                            div { class: "mx-auto mb-3.5 grid h-[42px] w-[42px] place-items-center rounded-full border border-[#6d3f49] bg-[#2b1720] text-lg font-extrabold text-[#f09aa9]", "!" }
                                            h2 { class: "mb-2 mt-0 text-base font-bold text-[#eef4fb]", "Unable to calculate properties" }
                                            p { class: "m-0 text-[13px] leading-6 text-[#9caabd]", "{error}" }
                                        }
                                    }
                                },
                            }
                        }
                    }
                }

                section { class: "mt-10 border-t border-[#213147] pt-8",
                    span { class: "text-xs font-bold text-[#4b96ff]", "BROWSER-LOCAL DESCRIPTORS" }
                    h2 { class: "mb-2 mt-2 text-xl font-bold text-slate-50", "Calculate molecular properties from SMILES" }
                    p { class: "m-0 max-w-[860px] text-sm leading-6 text-[#9caabd]", "COSMolKit parses the SMILES and calculates molecular formula, average molecular weight, monoisotopic exact mass, heavy atom count, hydrogen-bond donors and acceptors, TPSA, strict rotatable bonds, Crippen logP, and total formal charge without uploading the molecule." }
                }

                section { class: "mt-10 border-t border-[#213147] pt-8",
                    div { class: "flex items-start justify-between gap-6 max-[800px]:flex-col",
                        div {
                            span { class: "text-xs font-bold text-[#4b96ff]", "PYTHON BACKEND" }
                            h2 { class: "mb-1.5 mt-2 text-xl font-bold text-slate-50", "Calculate the same descriptors in Python" }
                            p { class: "m-0 max-w-[680px] text-sm leading-6 text-[#9caabd]", "The Python package exposes the same COSMolKit descriptor implementations used by this browser tool." }
                        }
                        a { class: "inline-flex shrink-0 items-center gap-2 rounded-md border border-[#2a3b52] bg-[#0c1828] px-3 py-2 text-xs font-semibold text-[#b8c5d6] no-underline hover:border-[#438ee9] hover:text-white", href: "https://pypi.org/project/cosmolkit/{cosmolkit_version}/", target: "_blank", rel: "noreferrer", "COSMolKit Python {cosmolkit_version}" MdiIcon { size: 14, path: MDI_OPEN_IN_NEW } }
                    }
                    div { class: "mt-6 min-w-0 overflow-hidden rounded-lg border border-[#213147] bg-[#081321]",
                        div { class: "flex min-h-11 items-center justify-between border-b border-[#213147] px-4",
                            span { class: "text-xs font-semibold text-[#9caabd]", "molecular_properties.py" }
                            button { r#type: "button", class: "cursor-pointer rounded-[5px] border border-[#30435b] bg-[#0c1828] px-2.5 py-1.5 text-[11px] font-semibold text-[#c5d1df] hover:border-[#438ee9] hover:text-white", onclick: move |_| copy_python(PYTHON_CODE.to_string(), toast), "Copy" }
                        }
                        pre { class: "m-0 max-h-[430px] overflow-auto p-5 text-[12px] leading-5 text-[#cbd8e8]", code { "{PYTHON_CODE}" } }
                    }
                }
            }
        }
    }
}

#[component]
fn PropertyCell(label: String, value: String, unit: String) -> Element {
    rsx! {
        div { class: "flex min-h-[92px] items-center justify-between gap-4 bg-[#111c2c] px-6 py-4 max-[480px]:px-[18px]",
            div {
                span { class: "block text-[11px] font-bold text-[#718299]", "{label}" }
                strong { class: "mt-1.5 block text-xl font-bold text-[#f1f6fc]", "{value}" }
            }
            if !unit.is_empty() {
                span { class: "shrink-0 text-[11px] font-semibold text-[#6f8095]", "{unit}" }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ethanol_properties_match_known_values() {
        let result = calculate("OCC").expect("ethanol descriptors");
        assert_eq!(result.formula, "C2H6O");
        assert_eq!(result.canonical_smiles, "CCO");
        assert_eq!(result.heavy_atoms, 3);
        assert_eq!(result.hbd, 1);
        assert_eq!(result.hba, 1);
        assert_eq!(result.rotatable_bonds, 0);
        assert_eq!(result.formal_charge, 0);
        assert!((result.molecular_weight - 46.069).abs() < 0.001);
        assert!((result.exact_mass - 46.041865).abs() < 0.00001);
        assert!((result.tpsa - 20.23).abs() < 0.01);
        assert!((result.logp - -0.0014).abs() < 0.0001);
    }

    #[test]
    fn properties_reject_invalid_smiles() {
        assert!(calculate("not smiles").is_err());
    }
}
