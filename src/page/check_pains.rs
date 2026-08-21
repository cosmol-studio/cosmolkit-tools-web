use cosmolkit::Molecule;
use dioxus::prelude::*;

use crate::component::Seo;

const DEFAULT_SMILES: &str = "O=C(C=Cc1ccc(O)cc1)c2ccc(O)cc2";
const EXAMPLES: [(&str, &str); 4] = [
    ("Curcumin fragment", DEFAULT_SMILES),
    ("Caffeine", "Cn1c(=O)c2c(ncn2C)n(C)c1=O"),
    ("Aspirin", "CC(=O)Oc1ccccc1C(=O)O"),
    ("Rhodanine", "O=C1CSC(=S)N1"),
];

#[derive(Clone, PartialEq)]
struct Preview {
    url: String,
    atom_count: usize,
    bond_count: usize,
}

fn preview_smiles(smiles: &str) -> Result<Preview, String> {
    let smiles = smiles.trim();
    if smiles.is_empty() {
        return Err("Enter a SMILES string to preview.".to_string());
    }
    if smiles.len() > 4096 {
        return Err("SMILES input is too long (maximum 4096 characters).".to_string());
    }
    let molecule = Molecule::from_smiles(smiles)
        .map_err(|error| format!("Could not parse this SMILES: {error}"))?;
    let atom_count = molecule.num_atoms();
    let bond_count = molecule.num_bonds();
    let svg = molecule
        .to_svg(720, 480)
        .map_err(|error| format!("Could not draw this molecule: {error}"))?;
    Ok(Preview {
        url: svg_data_url(&svg),
        atom_count,
        bond_count,
    })
}

fn svg_data_url(svg: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut encoded = String::with_capacity(svg.len() * 2);
    encoded.push_str("data:image/svg+xml;charset=utf-8,");
    for byte in svg.bytes() {
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

#[component]
pub fn CheckPains() -> Element {
    let mut smiles = use_signal(|| DEFAULT_SMILES.to_string());
    let mut preview = use_signal(|| preview_smiles(DEFAULT_SMILES));
    let cosmolkit_version = cosmolkit::version();

    rsx! {
        Seo {
            title: "Check PAINS — Implementation Status | COSMolKit",
            description: "Preview molecular structures for a future COSMolKit PAINS screening workflow. PAINS matching remains unavailable until validated core support is implemented.",
            canonical: "https://tools.cosmol.org/check-pains",
        }
        div { class: "min-h-screen uu-backdrop m-0 pt-[74px]",
            main { class: "mx-auto w-full max-w-6xl px-0 py-5 font-sans text-[#e8edf5] max-[800px]:px-3.5 max-[800px]:pb-[30px]",
                section {
                    div { class: "mb-6 flex items-end justify-between gap-8 max-[800px]:flex-col max-[800px]:items-start max-[800px]:gap-4",
                        div {
                            Link { class: "text-[13px] font-semibold text-[#7ab5ff] no-underline hover:text-[#b4d6ff]", to: crate::route::Route::ToolDirectory {}, "Back to tools" }
                            h1 { class: "mb-1.5 mt-2.5 text-[32px] leading-[1.2] font-bold text-slate-50 max-[800px]:text-[27px]", "Check PAINS" }
                            p { class: "m-0 max-w-[680px] text-[15px] leading-[1.6] text-[#9caabd]", "Prepare molecules for pan-assay interference pattern screening." }
                        }
                        div { class: "inline-flex shrink-0 items-center gap-2 rounded-md border border-[#5d5132] bg-[#282315] px-[11px] py-2 text-xs font-semibold text-[#d9bd72]",
                            span { class: "h-[7px] w-[7px] rounded-full bg-[#d2a93f]" }
                            "COSMolKit {cosmolkit_version} / Core support pending"
                        }
                    }

                    div { class: "grid min-h-[650px] grid-cols-[340px_minmax(0,1fr)] overflow-hidden rounded-lg border border-[#213147] bg-[#0c1727] shadow-[0_20px_55px_rgba(0,0,0,0.24)] max-[800px]:grid-cols-1",
                        aside { class: "border-r border-[#213147] bg-[#0a1524] p-6 max-[800px]:border-b max-[800px]:border-r-0 max-[480px]:p-[18px]",
                            div { class: "mb-6",
                                div { class: "flex items-center justify-between",
                                    label { class: "mb-[9px] block text-[13px] font-bold text-[#dce5f0]", r#for: "pains-smiles-input", "SMILES" }
                                    span { class: "text-[11px] text-[#6f8095]", "{smiles().len()} / 4096" }
                                }
                                textarea { id: "pains-smiles-input", class: "block min-h-[150px] w-full resize-y rounded-md border border-[#2a3b52] bg-[#07111f] px-3.5 py-[13px] font-mono text-[13px] leading-[1.65] text-[#eaf1fa] outline-none hover:border-[#438ee9] focus:border-[#438ee9] focus:ring-[3px] focus:ring-[#3082ff1f]", value: "{smiles}", maxlength: 4096, spellcheck: false, oninput: move |event| smiles.set(event.value()) }
                            }
                            div { class: "mb-6",
                                label { class: "mb-[9px] block text-[13px] font-bold text-[#dce5f0]", "Examples" }
                                div { class: "flex flex-wrap gap-[7px]",
                                    for (name, value) in EXAMPLES {
                                        button { r#type: "button", class: "cursor-pointer rounded-[5px] border border-[#2b3d54] bg-[#101e30] px-[9px] py-1.5 text-xs text-[#adbbcc] hover:border-[#438ee9] hover:text-[#eaf3ff]", onclick: move |_| { smiles.set(value.to_string()); preview.set(preview_smiles(value)); }, "{name}" }
                                    }
                                }
                            }
                            button { r#type: "button", class: "mb-3 h-[42px] w-full cursor-pointer rounded-md border border-[#30435b] bg-[#101e30] text-[13px] font-bold text-[#d5e2f1] hover:border-[#438ee9] hover:text-white", onclick: move |_| preview.set(preview_smiles(&smiles())), "Preview molecule" }
                            button { r#type: "button", class: "h-[42px] w-full cursor-not-allowed rounded-md border border-[#554a2e] bg-[#282315] text-[13px] font-bold text-[#8f8057] opacity-75", disabled: true, title: "PAINS matching is not available in COSMolKit Core {cosmolkit_version}", "Check PAINS" }
                            p { class: "mb-0 mt-[13px] text-center text-[11px] leading-5 text-[#8d805f]", "Screening remains disabled until COSMolKit Core provides a validated PAINS matcher." }
                        }

                        section { class: "grid min-w-0 grid-rows-[auto_1fr_auto] bg-[#111c2c]",
                            div { class: "flex min-h-16 items-center justify-between gap-5 border-b border-[#213147] px-[18px] py-3",
                                div {
                                    h2 { class: "mb-[3px] mt-0 text-sm font-bold text-[#eef4fb]", "Molecule preview" }
                                    if let Ok(molecule) = &*preview.read() { span { class: "text-[11px] text-[#718299]", "{molecule.atom_count} atoms  /  {molecule.bond_count} bonds" } }
                                }
                                span { class: "rounded-[5px] border border-[#5d5132] bg-[#282315] px-2 py-1 text-[10px] font-bold text-[#d9bd72]", "NOT AVAILABLE" }
                            }
                            div { id: "pains-preview-stage", class: "grid min-h-[500px] place-items-center bg-[#e9eef4] p-6 max-[800px]:min-h-[420px] max-[800px]:p-3",
                                match &*preview.read() {
                                    Ok(molecule) => rsx! { div { class: "grid h-full min-h-[440px] w-full place-items-center overflow-hidden border border-[#d5dde7] bg-white max-[800px]:min-h-[360px]", img { class: "block h-full max-h-full w-full object-contain", src: molecule.url.clone(), alt: "Molecule prepared for PAINS screening" } } },
                                    Err(error) => rsx! { div { class: "max-w-[460px] text-center", div { class: "mx-auto mb-3 grid h-11 w-11 place-items-center rounded-full border border-[#e6a9a9] bg-[#fff5f5] text-lg font-bold text-[#c83e3e]", "!" } h3 { class: "mb-2 mt-0 text-base font-bold text-[#172234]", "Unable to preview" } p { class: "m-0 text-[13px] leading-6 text-[#68778a]", "{error}" } } },
                                }
                            }
                            div { class: "border-t border-[#5d5132] bg-[#211d14] px-5 py-4",
                                div { class: "flex items-start gap-3",
                                    span { class: "mt-0.5 grid h-6 w-6 shrink-0 place-items-center rounded-full border border-[#806c37] text-xs font-bold text-[#d9bd72]", "i" }
                                    div { h3 { class: "m-0 text-[13px] font-bold text-[#ead69d]", "Not yet supported by COSMolKit Core" } p { class: "mb-0 mt-1 text-xs leading-5 text-[#a99b76]", "This page does not classify the molecule or return a synthetic result. The input and preview workflow is ready for the validated Core implementation." } }
                                }
                            }
                        }
                    }
                }

                section { class: "mt-10 grid grid-cols-[minmax(0,1fr)_300px] gap-8 border-t border-[#213147] pt-8 max-[800px]:grid-cols-1",
                    div {
                        span { class: "text-xs font-bold text-[#4b96ff]", "IMPLEMENTATION STATUS" }
                        h2 { class: "mb-2 mt-2 text-xl font-bold text-white", "Waiting for a validated Core matcher" }
                        p { class: "m-0 max-w-[700px] text-sm leading-6 text-[#9caabd]", "PAINS screening depends on a curated SMARTS catalog, matching semantics, and reproducible validation. It will be enabled here only after that behavior is part of COSMolKit Core." }
                    }
                    div { class: "border-l-2 border-[#d2a93f] pl-4",
                        span { class: "block text-[11px] font-bold text-[#8d805f]", "CURRENT CORE" }
                        code { class: "mt-2 block font-mono text-[13px] text-[#dce5f0]", "cosmolkit == {cosmolkit_version}" }
                        p { class: "mb-0 mt-2 text-xs leading-5 text-[#718299]", "No PAINS API exposed" }
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
    fn preview_uses_cosmolkit_parser_and_svg_renderer() {
        let preview = preview_smiles("c1ccccc1").unwrap();
        assert_eq!(preview.atom_count, 6);
        assert_eq!(preview.bond_count, 6);
        assert!(preview.url.starts_with("data:image/svg+xml"));
    }

    #[test]
    fn preview_rejects_invalid_smiles() {
        assert!(preview_smiles("C(").is_err());
    }
}
