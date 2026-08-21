use cosmol_viewer_core::{scene::Scene, shapes::Molecule as ViewerMolecule};
use cosmolkit::{EmbedParameters, Molecule, io::molblock};
use dioxus::prelude::*;

use crate::component::{MdiIcon, Seo, ToastManager, icon::MDI_OPEN_IN_NEW};
#[cfg(target_arch = "wasm32")]
use crate::component::{Viewer, get_viewer};

const CANVAS_ID: &str = "conformer-viewer-canvas";
const DEFAULT_SMILES: &str = "CC(=O)Oc1ccccc1C(=O)O";
const MAX_INPUT_LENGTH: usize = 4096;
const MAX_ATOMS_BEFORE_HYDROGENS: usize = 256;
const EXAMPLES: [(&str, &str); 4] = [
    ("Aspirin", DEFAULT_SMILES),
    ("Caffeine", "Cn1c(=O)c2c(ncn2C)n(C)c1=O"),
    ("Ibuprofen", "CC(C)Cc1ccc(cc1)[C@@H](C)C(=O)O"),
    ("Menthol", "CC(C)[C@H]1CC[C@@H](C)C[C@H]1O"),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EmbedPreset {
    EtkdgV3,
    EtkdgV2,
    Kdg,
}

impl EmbedPreset {
    const ALL: [Self; 3] = [Self::EtkdgV3, Self::EtkdgV2, Self::Kdg];

    fn id(self) -> &'static str {
        match self {
            Self::EtkdgV3 => "etkdg-v3",
            Self::EtkdgV2 => "etkdg-v2",
            Self::Kdg => "kdg",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::EtkdgV3 => "ETKDG v3",
            Self::EtkdgV2 => "ETKDG v2",
            Self::Kdg => "KDG",
        }
    }

    fn from_id(id: &str) -> Self {
        Self::ALL
            .into_iter()
            .find(|preset| preset.id() == id)
            .unwrap_or(Self::EtkdgV3)
    }

    fn parameters(self) -> EmbedParameters {
        match self {
            Self::EtkdgV3 => EmbedParameters::etkdg_v3(),
            Self::EtkdgV2 => EmbedParameters::etkdg_v2(),
            Self::Kdg => EmbedParameters::kdg(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OutputTab {
    Sdf,
    Pdb,
}

#[derive(Clone, Debug, PartialEq)]
struct ConformerResult {
    sdf: String,
    pdb: String,
    sdf_url: String,
    pdb_url: String,
    atom_count: usize,
    bond_count: usize,
    conformer_count: usize,
    hydrogen_count: usize,
}

fn generate_conformer(
    smiles: &str,
    preset: EmbedPreset,
    seed: i32,
    add_hydrogens: bool,
) -> Result<(ConformerResult, Scene), String> {
    let smiles = smiles.trim();
    if smiles.is_empty() {
        return Err("Enter a SMILES string to generate a conformer.".to_string());
    }
    if smiles.len() > MAX_INPUT_LENGTH {
        return Err(format!(
            "SMILES input is too long (maximum {MAX_INPUT_LENGTH} characters)."
        ));
    }
    if seed < 0 {
        return Err("The browser build requires a non-negative random seed.".to_string());
    }

    let base = Molecule::from_smiles(smiles)
        .map_err(|error| format!("Could not parse this SMILES: {error}"))?;
    if base.num_atoms() > MAX_ATOMS_BEFORE_HYDROGENS {
        return Err(format!(
            "This browser tool accepts at most {MAX_ATOMS_BEFORE_HYDROGENS} atoms before adding hydrogens."
        ));
    }

    let base_atom_count = base.num_atoms();
    let molecule = if add_hydrogens {
        base.with_hydrogens()
            .map_err(|error| format!("Could not add hydrogens: {error}"))?
    } else {
        base
    };
    let hydrogen_count = molecule.num_atoms().saturating_sub(base_atom_count);

    let mut params = preset.parameters();
    params.random_seed = seed;
    params.num_threads = 1;
    let molecule = molecule
        .with_3d_conformer_with_params(params)
        .map_err(|error| format!("Could not generate a 3D conformer: {error}"))?;
    let conformer_count = molecule.conformers_3d().len();
    if conformer_count == 0 {
        return Err("COSMolKit could not find a valid 3D conformer for this molecule.".to_string());
    }

    let params = molblock::MolBlockWriteParams {
        format: molblock::SdfFormat::V3000,
        ..Default::default()
    };
    let sdf = molblock::mol_to_sdf_record_with_params(&molecule, &params)
        .map_err(|error| format!("Could not write the generated SDF: {error}"))?;
    let pdb = molecule.to_pdb_block(-1, 0);

    let viewer_molecule = ViewerMolecule::from_cosmolkit(&molecule)
        .map_err(|error| format!("Could not prepare the 3D preview: {error}"))?
        .centered()
        .set_outline(true, "#ffffff", 0.035);
    let center = viewer_molecule.get_center();
    let mut scene = Scene::new();
    scene.set_scale(0.96);
    scene.recenter(center);
    scene.add_shape(viewer_molecule);
    scene.set_background_color("#07111f");
    scene.set_zoom_disabled(true);

    Ok((
        ConformerResult {
            sdf_url: data_url("chemical/x-mdl-sdfile", &sdf),
            pdb_url: data_url("chemical/x-pdb", &pdb),
            sdf,
            pdb,
            atom_count: molecule.num_atoms(),
            bond_count: molecule.num_bonds(),
            conformer_count,
            hydrogen_count,
        },
        scene,
    ))
}

#[cfg(target_arch = "wasm32")]
fn empty_scene() -> Scene {
    let mut scene = Scene::new();
    scene.set_background_color("#07111f");
    scene.set_zoom_disabled(true);
    scene
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

#[cfg(target_arch = "wasm32")]
fn start_generation(
    smiles: String,
    preset: EmbedPreset,
    seed: i32,
    add_hydrogens: bool,
    mut busy: Signal<bool>,
    mut result: Signal<Option<Result<ConformerResult, String>>>,
    viewer: Signal<Option<Viewer>>,
    mut toast: ToastManager,
) {
    busy.set(true);
    wasm_bindgen_futures::spawn_local(async move {
        gloo_timers::future::TimeoutFuture::new(16).await;
        let generated = generate_conformer(&smiles, preset, seed, add_hydrogens);
        let output = match generated {
            Ok((output, scene)) => match viewer.read().as_ref() {
                Some(viewer) => viewer.update_scene(&scene).map(|()| output),
                None => Err("The 3D viewer is still starting. Try again in a moment.".to_string()),
            },
            Err(error) => Err(error),
        };
        if output.is_ok() {
            toast.success("3D conformer generated.");
        }
        result.set(Some(output));
        busy.set(false);
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn start_generation(
    smiles: String,
    preset: EmbedPreset,
    seed: i32,
    add_hydrogens: bool,
    mut busy: Signal<bool>,
    mut result: Signal<Option<Result<ConformerResult, String>>>,
    mut toast: ToastManager,
) {
    busy.set(true);
    let output = generate_conformer(&smiles, preset, seed, add_hydrogens).map(|(output, _)| output);
    if output.is_ok() {
        toast.success("3D conformer generated.");
    }
    result.set(Some(output));
    busy.set(false);
}

#[component]
pub fn ConformerGenerator() -> Element {
    let mut smiles = use_signal(|| DEFAULT_SMILES.to_string());
    let mut preset = use_signal(|| EmbedPreset::EtkdgV3);
    let mut seed = use_signal(|| 61453_i32);
    let mut add_hydrogens = use_signal(|| true);
    let mut output_tab = use_signal(|| OutputTab::Sdf);
    #[allow(unused_mut)]
    let mut busy = use_signal(|| true);
    #[allow(unused_mut)]
    let mut result = use_signal(|| None::<Result<ConformerResult, String>>);
    let toast = use_context::<ToastManager>();
    let cosmolkit_version = cosmolkit::version();

    #[cfg(target_arch = "wasm32")]
    let mut viewer: Signal<Option<Viewer>> = use_signal(|| None);

    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        wasm_bindgen_futures::spawn_local(async move {
            match get_viewer(empty_scene(), CANVAS_ID).await {
                Ok(initialized) => {
                    viewer.set(Some(initialized));
                    start_generation(
                        DEFAULT_SMILES.to_string(),
                        EmbedPreset::EtkdgV3,
                        61453,
                        true,
                        busy,
                        result,
                        viewer,
                        toast,
                    );
                }
                Err(error) => {
                    result.set(Some(Err(format!(
                        "Could not start the 3D viewer: {error:?}"
                    ))));
                    busy.set(false);
                }
            }
        });
    });

    #[cfg(not(target_arch = "wasm32"))]
    use_effect(move || {
        start_generation(
            DEFAULT_SMILES.to_string(),
            EmbedPreset::EtkdgV3,
            61453,
            true,
            busy,
            result,
            toast,
        );
    });

    let current_output = result().and_then(Result::ok);

    rsx! {
        Seo {
            title: "SMILES to 3D Conformer — Browser ETKDG Generator | COSMolKit",
            description: "Convert SMILES to a 3D molecular conformer with ETKDG v2, ETKDG v3, or KDG. Preview the 3D molecule and export SDF or PDB coordinates locally.",
            canonical: "https://tools.cosmol.org/conformer-generator",
        }
        div { class: "min-h-screen uu-backdrop m-0 pt-[74px]",
            main { class: "mx-auto w-full max-w-6xl px-0 py-5 font-sans text-[#e8edf5] max-[820px]:px-3.5 max-[820px]:pb-[30px]",
                section { class: "w-full",
                    div { class: "mb-6 flex items-end justify-between gap-8 max-[820px]:flex-col max-[820px]:items-start max-[820px]:gap-4",
                        div {
                            Link { class: "text-[13px] font-semibold text-[#7ab5ff] no-underline hover:text-[#b4d6ff]", to: crate::route::Route::ToolDirectory {}, "Back to tools" }
                            h1 { class: "mb-1.5 mt-2.5 text-[32px] leading-[1.2] font-bold text-slate-50 max-[820px]:text-[27px]", "3D conformer generator" }
                            p { class: "m-0 max-w-[700px] text-[15px] leading-[1.6] text-[#9caabd]", "Generate deterministic 3D molecular coordinates entirely in your browser with distance geometry." }
                        }
                        a {
                            class: "inline-flex shrink-0 items-center gap-2 rounded-md border border-[#23344a] bg-[#0c1828] px-[11px] py-2 text-xs font-semibold text-[#b8c5d6] no-underline hover:border-[#438ee9] hover:text-white",
                            href: "https://crates.io/crates/cosmolkit", target: "_blank", rel: "noreferrer",
                            "COSMolKit {cosmolkit_version} / Rust / WASM"
                            MdiIcon { size: 14, path: MDI_OPEN_IN_NEW }
                        }
                    }

                    div { class: "grid min-h-[720px] grid-cols-[350px_minmax(0,1fr)] overflow-hidden rounded-lg border border-[#213147] bg-[#0c1727] shadow-[0_20px_55px_rgba(0,0,0,0.24)] max-[820px]:grid-cols-1",
                        aside { class: "border-r border-[#213147] bg-[#0a1524] p-6 max-[820px]:border-r-0 max-[820px]:border-b max-[480px]:p-[18px]",
                            div { class: "mb-6",
                                div { class: "flex items-center justify-between",
                                    label { class: "mb-[9px] block text-[13px] font-bold text-[#dce5f0]", r#for: "conformer-smiles", "SMILES" }
                                    span { class: "text-[11px] text-[#6f8095]", "{smiles().len()} / {MAX_INPUT_LENGTH}" }
                                }
                                textarea {
                                    id: "conformer-smiles", maxlength: MAX_INPUT_LENGTH, spellcheck: false,
                                    class: "block min-h-[138px] w-full resize-y rounded-md border border-[#2a3b52] bg-[#07111f] px-3.5 py-[13px] font-mono text-[13px] leading-[1.65] text-[#eaf1fa] outline-none hover:border-[#438ee9] focus:border-[#438ee9] focus:ring-[3px] focus:ring-[#3082ff1f]",
                                    value: "{smiles}", oninput: move |event| smiles.set(event.value()),
                                }
                            }

                            div { class: "mb-6",
                                label { class: "mb-[9px] block text-[13px] font-bold text-[#dce5f0]", "Examples" }
                                div { class: "flex flex-wrap gap-[7px]",
                                    for (name, value) in EXAMPLES {
                                        button {
                                            r#type: "button",
                                            disabled: busy(),
                                            class: "cursor-pointer rounded-[5px] border border-[#2b3d54] bg-[#101e30] px-[9px] py-1.5 text-xs text-[#adbbcc] hover:border-[#438ee9] hover:text-[#eaf3ff] disabled:cursor-wait disabled:opacity-50",
                                            onclick: move |_| {
                                                let selected = value.to_string();
                                                smiles.set(selected.clone());
                                                #[cfg(target_arch = "wasm32")]
                                                start_generation(selected, preset(), seed(), add_hydrogens(), busy, result, viewer, toast);
                                                #[cfg(not(target_arch = "wasm32"))]
                                                start_generation(selected, preset(), seed(), add_hydrogens(), busy, result, toast);
                                            },
                                            "{name}"
                                        }
                                    }
                                }
                            }

                            div { class: "mb-4",
                                label { class: "mb-[9px] block text-[13px] font-bold text-[#dce5f0]", r#for: "embed-preset", "Embedding method" }
                                div { class: "relative",
                                    select {
                                        id: "embed-preset",
                                        class: "h-[46px] w-full cursor-pointer appearance-none rounded-md border border-[#2a3b52] bg-[#07111f] px-3.5 pr-11 text-[13px] font-semibold text-[#eaf1fa] outline-none hover:border-[#438ee9] focus:border-[#438ee9] focus:ring-[3px] focus:ring-[#3082ff1f]",
                                        value: preset().id(), onchange: move |event| preset.set(EmbedPreset::from_id(&event.value())),
                                        for option in EmbedPreset::ALL { option { value: option.id(), "{option.label()}" } }
                                    }
                                    span { class: "pointer-events-none absolute top-1/2 right-4 -translate-y-1/2 text-xs text-[#8fa0b5]", "v" }
                                }
                            }

                            div { class: "mb-4",
                                label { class: "mb-[9px] block text-[13px] font-bold text-[#dce5f0]", r#for: "random-seed", "Random seed" }
                                input {
                                    id: "random-seed", r#type: "number", min: 0, max: 2147483647_i32, step: 1,
                                    class: "h-[46px] w-full rounded-md border border-[#2a3b52] bg-[#07111f] px-3.5 font-mono text-[13px] text-[#eaf1fa] outline-none hover:border-[#438ee9] focus:border-[#438ee9] focus:ring-[3px] focus:ring-[#3082ff1f]",
                                    value: "{seed}",
                                    oninput: move |event| if let Ok(value) = event.value().parse::<i32>() { seed.set(value.max(0)); },
                                }
                            }

                            label { class: "mb-6 flex cursor-pointer items-center justify-between gap-4 rounded-md border border-[#2a3b52] bg-[#0d1a2b] px-3.5 py-3",
                                span {
                                    span { class: "block text-[13px] font-bold text-[#dce5f0]", "Add explicit hydrogens" }
                                    span { class: "mt-0.5 block text-[11px] text-[#718299]", "Recommended for all-atom geometry" }
                                }
                                input { r#type: "checkbox", class: "peer sr-only", checked: add_hydrogens(), onchange: move |event| add_hydrogens.set(event.checked()) }
                                span { class: "relative h-6 w-11 shrink-0 rounded-full border border-[#3a4b61] bg-[#172437] transition-colors after:absolute after:top-[3px] after:left-[3px] after:h-4 after:w-4 after:rounded-full after:bg-[#8d9aad] after:transition-transform peer-checked:border-[#398cf6] peer-checked:bg-[#267de8] peer-checked:after:translate-x-5 peer-checked:after:bg-white" }
                            }

                            button {
                                r#type: "button", disabled: busy(),
                                class: "h-[42px] w-full cursor-pointer rounded-md border border-[#398cf6] bg-[#267de8] text-[13px] font-bold text-white shadow-[0_8px_24px_rgba(38,125,232,0.2)] hover:bg-[#358bf3] active:translate-y-px disabled:cursor-wait disabled:border-[#2a527e] disabled:bg-[#173f6d] disabled:text-[#9db8d8]",
                                onclick: move |_| {
                                    #[cfg(target_arch = "wasm32")]
                                    start_generation(smiles(), preset(), seed(), add_hydrogens(), busy, result, viewer, toast);
                                    #[cfg(not(target_arch = "wasm32"))]
                                    start_generation(smiles(), preset(), seed(), add_hydrogens(), busy, result, toast);
                                },
                                if busy() { "Generating..." } else { "Generate conformer" }
                            }
                            p { class: "mt-[13px] mb-0 text-center text-[11px] text-[#718299]", "Single-threaded WASM / deterministic seed / local execution" }
                        }

                        section { class: "grid min-w-0 grid-rows-[auto_430px_auto_minmax(220px,1fr)] bg-[#111c2c]",
                            div { class: "flex min-h-16 items-center justify-between gap-5 border-b border-[#213147] px-[18px] py-3 max-[560px]:flex-col max-[560px]:items-start",
                                div {
                                    h2 { class: "mt-0 mb-[3px] text-sm font-bold text-[#eef4fb]", "3D preview" }
                                    if let Some(value) = &current_output {
                                        span { class: "text-[11px] text-[#718299]", "{value.atom_count} atoms  /  {value.bond_count} bonds  /  {value.conformer_count} conformer" }
                                    } else { span { class: "text-[11px] text-[#718299]", "Drag to rotate" } }
                                }
                                if let Some(value) = &current_output {
                                    span { class: "rounded-[5px] border border-[#285b4d] bg-[#0d2923] px-2.5 py-1.5 text-[10px] font-bold text-[#8ee0c4]", "+{value.hydrogen_count} H" }
                                }
                            }

                            div { class: "relative min-h-0 overflow-hidden bg-[#07111f]",
                                canvas { id: CANVAS_ID, class: "block h-full w-full touch-none" }
                                if busy() {
                                    div { class: "absolute inset-0 grid place-items-center bg-[#07111f]/85",
                                        div { class: "text-center",
                                            div { class: "mx-auto mb-3 h-7 w-7 animate-spin rounded-full border-2 border-[#29415f] border-t-[#4b96ff]" }
                                            span { class: "text-xs font-semibold text-[#9fb0c5]", "Computing distance geometry" }
                                        }
                                    }
                                } else if let Some(Err(error)) = result.read().as_ref() {
                                    div { class: "absolute inset-0 grid place-items-center bg-[#07111f]/92 p-6 text-center",
                                        div { class: "max-w-[480px]",
                                            div { class: "mx-auto mb-3 grid h-10 w-10 place-items-center rounded-full border border-[#713d46] bg-[#301820] font-extrabold text-[#f0a7b2]", "!" }
                                            h3 { class: "mb-1.5 mt-0 text-sm font-bold text-white", "Unable to generate" }
                                            p { class: "m-0 text-xs leading-5 text-[#9caabd]", "{error}" }
                                        }
                                    }
                                }
                            }

                            div { class: "flex items-center justify-between gap-4 border-y border-[#213147] bg-[#0d1929] px-[18px] py-2.5 max-[480px]:flex-col max-[480px]:items-stretch",
                                div { class: "grid grid-cols-2 rounded-md border border-[#2a3b52] bg-[#07111f] p-1",
                                    button { r#type: "button", class: if output_tab() == OutputTab::Sdf { "h-8 rounded-[4px] bg-[#203650] px-4 text-xs font-bold text-white" } else { "h-8 cursor-pointer rounded-[4px] px-4 text-xs font-semibold text-[#8495aa] hover:text-white" }, onclick: move |_| output_tab.set(OutputTab::Sdf), "SDF V3000" }
                                    button { r#type: "button", class: if output_tab() == OutputTab::Pdb { "h-8 rounded-[4px] bg-[#203650] px-4 text-xs font-bold text-white" } else { "h-8 cursor-pointer rounded-[4px] px-4 text-xs font-semibold text-[#8495aa] hover:text-white" }, onclick: move |_| output_tab.set(OutputTab::Pdb), "PDB" }
                                }
                                if let Some(value) = &current_output {
                                    div { class: "flex gap-2",
                                        button {
                                            r#type: "button", class: "h-[34px] flex-1 cursor-pointer rounded-[5px] border border-[#30435b] bg-[#0c1828] px-[11px] text-xs font-semibold text-[#c5d1df] hover:border-[#438ee9] hover:text-white",
                                            onclick: { let text = if output_tab() == OutputTab::Sdf { value.sdf.clone() } else { value.pdb.clone() }; move |_| copy_text(text.clone(), "Coordinates copied.", toast) },
                                            "Copy"
                                        }
                                        a {
                                            class: "inline-flex h-[34px] flex-1 items-center justify-center rounded-[5px] border border-[#30435b] bg-[#0c1828] px-[11px] text-xs font-semibold text-[#c5d1df] no-underline hover:border-[#438ee9] hover:text-white",
                                            href: if output_tab() == OutputTab::Sdf { value.sdf_url.clone() } else { value.pdb_url.clone() },
                                            download: if output_tab() == OutputTab::Sdf { "conformer.sdf" } else { "conformer.pdb" },
                                            "Download"
                                        }
                                    }
                                }
                            }

                            pre { class: "m-0 min-h-[220px] overflow-auto bg-[#091422] p-[18px] font-mono text-[11px] leading-[1.6] text-[#aebed1]",
                                if let Some(value) = &current_output {
                                    if output_tab() == OutputTab::Sdf { "{value.sdf}" } else { "{value.pdb}" }
                                } else { "Generated coordinates will appear here." }
                            }
                        }
                    }
                }

                section { class: "mt-10 border-t border-[#213147] pt-8",
                    span { class: "text-xs font-bold text-[#4b96ff]", "3D CONFORMER WORKFLOW" }
                    h2 { class: "mb-2 mt-2 text-xl font-bold text-slate-50", "Convert SMILES to a 3D conformer in the browser" }
                    p { class: "m-0 max-w-[860px] text-sm leading-6 text-[#9caabd]",
                        "Choose ETKDG v3, ETKDG v2, or KDG to generate one 3D conformer from SMILES. Adding explicit hydrogens is recommended for all-atom geometry. This browser 3D molecule generator runs locally in single-threaded WebAssembly and supports SMILES to SDF V3000 and SMILES to PDB export workflows."
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
    fn generates_deterministic_ethanol_coordinates() {
        let (result, _) = generate_conformer("CCO", EmbedPreset::EtkdgV3, 61453, true)
            .expect("ethanol conformer");
        assert_eq!(result.conformer_count, 1);
        assert_eq!(result.atom_count, 9);
        assert_eq!(result.hydrogen_count, 6);
        assert!(result.sdf.contains("V3000"));
        assert!(result.pdb.contains("HETATM"));
    }

    #[test]
    fn rejects_negative_seed_before_embedding() {
        assert!(
            generate_conformer("CCO", EmbedPreset::EtkdgV3, -1, true)
                .unwrap_err()
                .contains("non-negative")
        );
    }
}
