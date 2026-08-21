use cosmolkit::{Molecule, io::molblock};
use dioxus::prelude::*;

use crate::component::{
    MdiIcon, Seo, ToastManager,
    icon::{MDI_CHEVRON_DOWN, MDI_OPEN_IN_NEW},
};

const MAX_INPUT_LENGTH: usize = 2_000_000;
const DEFAULT_SMILES: &str = "CC(=O)Oc1ccccc1C(=O)O";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum InputFormat {
    Smiles,
    MolSdf,
    Mol2,
    Pdb,
    Mmcif,
    Xyz,
}

impl InputFormat {
    const ALL: [Self; 6] = [
        Self::Smiles,
        Self::MolSdf,
        Self::Mol2,
        Self::Pdb,
        Self::Mmcif,
        Self::Xyz,
    ];

    fn id(self) -> &'static str {
        match self {
            Self::Smiles => "smiles",
            Self::MolSdf => "mol-sdf",
            Self::Mol2 => "mol2",
            Self::Pdb => "pdb",
            Self::Mmcif => "mmcif",
            Self::Xyz => "xyz",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Smiles => "SMILES",
            Self::MolSdf => "MOL / SDF",
            Self::Mol2 => "Tripos MOL2",
            Self::Pdb => "PDB",
            Self::Mmcif => "mmCIF",
            Self::Xyz => "XYZ",
        }
    }

    fn detail(self) -> &'static str {
        match self {
            Self::Smiles => "Line notation",
            Self::MolSdf => "V2000 or V3000",
            Self::Mol2 => "Tripos structure",
            Self::Pdb => "Protein Data Bank",
            Self::Mmcif => "PDBx/mmCIF",
            Self::Xyz => "Coordinates only",
        }
    }

    fn accept(self) -> &'static str {
        match self {
            Self::Smiles => ".smi,.smiles,.txt",
            Self::MolSdf => ".mol,.sdf",
            Self::Mol2 => ".mol2",
            Self::Pdb => ".pdb,.ent",
            Self::Mmcif => ".cif,.mmcif",
            Self::Xyz => ".xyz",
        }
    }

    fn from_id(value: &str) -> Self {
        Self::ALL
            .into_iter()
            .find(|format| format.id() == value)
            .unwrap_or(Self::Smiles)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OutputFormat {
    Smiles,
    MolV2000,
    MolV3000,
    SdfV2000,
    SdfV3000,
    Pdb,
    Svg,
}

impl OutputFormat {
    const ALL: [Self; 7] = [
        Self::Smiles,
        Self::MolV2000,
        Self::MolV3000,
        Self::SdfV2000,
        Self::SdfV3000,
        Self::Pdb,
        Self::Svg,
    ];

    fn id(self) -> &'static str {
        match self {
            Self::Smiles => "smiles",
            Self::MolV2000 => "mol-v2000",
            Self::MolV3000 => "mol-v3000",
            Self::SdfV2000 => "sdf-v2000",
            Self::SdfV3000 => "sdf-v3000",
            Self::Pdb => "pdb",
            Self::Svg => "svg",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Smiles => "SMILES",
            Self::MolV2000 => "MOL V2000",
            Self::MolV3000 => "MOL V3000",
            Self::SdfV2000 => "SDF V2000",
            Self::SdfV3000 => "SDF V3000",
            Self::Pdb => "PDB",
            Self::Svg => "SVG depiction",
        }
    }

    fn extension(self) -> &'static str {
        match self {
            Self::Smiles => "smi",
            Self::MolV2000 | Self::MolV3000 => "mol",
            Self::SdfV2000 | Self::SdfV3000 => "sdf",
            Self::Pdb => "pdb",
            Self::Svg => "svg",
        }
    }

    fn mime(self) -> &'static str {
        match self {
            Self::Smiles => "chemical/x-daylight-smiles",
            Self::MolV2000 | Self::MolV3000 => "chemical/x-mdl-molfile",
            Self::SdfV2000 | Self::SdfV3000 => "chemical/x-mdl-sdfile",
            Self::Pdb => "chemical/x-pdb",
            Self::Svg => "image/svg+xml",
        }
    }

    fn from_id(value: &str) -> Self {
        Self::ALL
            .into_iter()
            .find(|format| format.id() == value)
            .unwrap_or(Self::SdfV2000)
    }

    fn as_input(self) -> Option<InputFormat> {
        match self {
            Self::Smiles => Some(InputFormat::Smiles),
            Self::MolV2000 | Self::MolV3000 | Self::SdfV2000 | Self::SdfV3000 => {
                Some(InputFormat::MolSdf)
            }
            Self::Pdb => Some(InputFormat::Pdb),
            Self::Svg => None,
        }
    }
}

#[derive(Clone, PartialEq)]
struct ConversionResult {
    content: String,
    download_url: String,
    atom_count: usize,
    bond_count: usize,
}

fn output_for_input(format: InputFormat) -> Option<OutputFormat> {
    match format {
        InputFormat::Smiles => Some(OutputFormat::Smiles),
        InputFormat::MolSdf => Some(OutputFormat::SdfV2000),
        InputFormat::Pdb => Some(OutputFormat::Pdb),
        InputFormat::Mol2 | InputFormat::Mmcif | InputFormat::Xyz => None,
    }
}

fn parse_molecule(input: &str, format: InputFormat) -> Result<Molecule, String> {
    match format {
        InputFormat::Smiles => Molecule::from_smiles(input.trim())
            .map_err(|error| format!("Could not parse SMILES: {error}")),
        InputFormat::MolSdf => Molecule::from_mol_block(input)
            .map_err(|error| format!("Could not parse MOL/SDF: {error}")),
        InputFormat::Mol2 => cosmolkit::read_mol2_from_str(input)
            .map_err(|error| format!("Could not parse MOL2: {error}"))?
            .map(|record| record.molecule)
            .ok_or_else(|| "The MOL2 input does not contain a molecule record.".to_string()),
        InputFormat::Pdb => {
            Molecule::from_pdb_block(input).map_err(|error| format!("Could not parse PDB: {error}"))
        }
        InputFormat::Mmcif => Molecule::from_mmcif_block(input)
            .map_err(|error| format!("Could not parse mmCIF: {error}")),
        InputFormat::Xyz => {
            Molecule::from_xyz_block(input).map_err(|error| format!("Could not parse XYZ: {error}"))
        }
    }
}

fn write_molecule(molecule: &Molecule, format: OutputFormat) -> Result<String, String> {
    let write_error =
        |error: molblock::MolWriteError| format!("Could not write {}: {error}", format.label());
    match format {
        OutputFormat::Smiles => molecule
            .to_smiles(true)
            .map_err(|error| format!("Could not write SMILES: {error}")),
        OutputFormat::MolV2000 => molblock::mol_to_v2000_block(molecule).map_err(write_error),
        OutputFormat::MolV3000 => molblock::mol_to_v3000_block(molecule).map_err(write_error),
        OutputFormat::SdfV2000 | OutputFormat::SdfV3000 => {
            let params = molblock::MolBlockWriteParams {
                format: if format == OutputFormat::SdfV2000 {
                    molblock::SdfFormat::V2000
                } else {
                    molblock::SdfFormat::V3000
                },
                ..Default::default()
            };
            molblock::mol_to_sdf_record_with_params(molecule, &params).map_err(write_error)
        }
        OutputFormat::Pdb => Ok(molecule.to_pdb_block(-1, 0)),
        OutputFormat::Svg => molecule
            .to_svg(720, 480)
            .map_err(|error| format!("Could not draw SVG: {error}")),
    }
}

fn convert(
    input: &str,
    input_format: InputFormat,
    output_format: OutputFormat,
) -> Result<ConversionResult, String> {
    if input.trim().is_empty() {
        return Err("Paste a molecular record or choose a file to convert.".to_string());
    }
    if input.len() > MAX_INPUT_LENGTH {
        return Err("Input is too large (maximum 2,000,000 characters).".to_string());
    }

    let molecule = parse_molecule(input, input_format)?;
    let atom_count = molecule.num_atoms();
    let bond_count = molecule.num_bonds();
    let content = write_molecule(&molecule, output_format)?;
    let download_url = data_url(output_format.mime(), &content);

    Ok(ConversionResult {
        content,
        download_url,
        atom_count,
        bond_count,
    })
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

fn example_input(format: InputFormat) -> Result<String, String> {
    match format {
        InputFormat::Smiles => Ok(DEFAULT_SMILES.to_string()),
        InputFormat::MolSdf => {
            let molecule = Molecule::from_smiles("CCO")
                .map_err(|error| format!("Could not create the MOL example: {error}"))?;
            molblock::mol_to_v2000_block(&molecule)
                .map_err(|error| format!("Could not write the MOL example: {error}"))
        }
        InputFormat::Mol2 => Ok(r#"@<TRIPOS>MOLECULE
Ethanol
3 2 0 0 0
SMALL
NO_CHARGES

@<TRIPOS>ATOM
1 C1 0.0000 0.0000 0.0000 C.3 1 ETO 0.0000
2 C2 1.5200 0.0000 0.0000 C.3 1 ETO 0.0000
3 O1 2.1200 1.2100 0.0000 O.3 1 ETO 0.0000
@<TRIPOS>BOND
1 1 2 1
2 2 3 1
"#
        .to_string()),
        InputFormat::Pdb => Ok(
            r#"HETATM    1  O   HOH A   1       0.000   0.000   0.000  1.00 20.00           O
HETATM    2  H1  HOH A   1       0.957   0.000   0.000  1.00 20.00           H
HETATM    3  H2  HOH A   1      -0.240   0.927   0.000  1.00 20.00           H
CONECT    1    2    3
END
"#
            .to_string(),
        ),
        InputFormat::Mmcif => Ok(r#"data_water
loop_
_atom_site.group_PDB
_atom_site.id
_atom_site.type_symbol
_atom_site.label_atom_id
_atom_site.label_alt_id
_atom_site.label_comp_id
_atom_site.label_asym_id
_atom_site.label_entity_id
_atom_site.label_seq_id
_atom_site.pdbx_PDB_ins_code
_atom_site.Cartn_x
_atom_site.Cartn_y
_atom_site.Cartn_z
_atom_site.occupancy
_atom_site.B_iso_or_equiv
_atom_site.auth_seq_id
_atom_site.auth_comp_id
_atom_site.auth_asym_id
_atom_site.auth_atom_id
HETATM 1 O O . HOH A 1 1 ? 0.000 0.000 0.000 1.00 20.00 1 HOH A O
HETATM 2 H H1 . HOH A 1 1 ? 0.957 0.000 0.000 1.00 20.00 1 HOH A H1
HETATM 3 H H2 . HOH A 1 1 ? -0.240 0.927 0.000 1.00 20.00 1 HOH A H2
"#
        .to_string()),
        InputFormat::Xyz => Ok(
            "3\nwater\nO 0.000 0.000 0.000\nH 0.957 0.000 0.000\nH -0.240 0.927 0.000\n"
                .to_string(),
        ),
    }
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
            character => literal.push(character),
        }
    }
    literal.push('"');
    literal
}

fn python_example(input: &str, input_format: InputFormat, output_format: OutputFormat) -> String {
    let source = if input_format == InputFormat::Smiles {
        input.trim()
    } else {
        input
    };
    let constructor = match input_format {
        InputFormat::Smiles => "Molecule.from_smiles(source)",
        InputFormat::MolSdf => "Molecule.read_mol_from_str(source)",
        InputFormat::Mol2 => "Molecule.read_mol2_from_str(source)",
        InputFormat::Pdb => "Molecule.from_pdb_block(source)",
        InputFormat::Mmcif => "Molecule.from_mmcif_block(source)",
        InputFormat::Xyz => "Molecule.from_xyz_block(source)",
    };
    let (preparation, writer) = match output_format {
        OutputFormat::Smiles => ("", "output = mol.to_smiles()".to_string()),
        OutputFormat::MolV2000 => (
            PYTHON_PREPARE_2D,
            "output = mol_2d.to_2d_sdf_string(format=\"v2000\").removesuffix(\"$$$$\\n\")"
                .to_string(),
        ),
        OutputFormat::MolV3000 => (
            PYTHON_PREPARE_2D,
            "output = mol_2d.to_2d_sdf_string(format=\"v3000\").removesuffix(\"$$$$\\n\")"
                .to_string(),
        ),
        OutputFormat::SdfV2000 => (
            PYTHON_PREPARE_2D,
            "output = mol_2d.to_2d_sdf_string(format=\"v2000\")".to_string(),
        ),
        OutputFormat::SdfV3000 => (
            PYTHON_PREPARE_2D,
            "output = mol_2d.to_2d_sdf_string(format=\"v3000\")".to_string(),
        ),
        OutputFormat::Pdb => ("", "output = mol.to_pdb_block()".to_string()),
        OutputFormat::Svg => (
            PYTHON_PREPARE_2D,
            "output = mol_2d.to_svg(width=720, height=480)".to_string(),
        ),
    };
    format!(
        "from pathlib import Path\nfrom cosmolkit import Molecule\n\nsource = {}\nmol = {constructor}\n{preparation}{writer}\n\nPath(\"molecule.{}\").write_text(output, encoding=\"utf-8\")",
        python_string_literal(source),
        output_format.extension(),
    )
}

const PYTHON_PREPARE_2D: &str = "try:\n    mol_2d = mol.with_2d_coordinates()\nexcept (ValueError, NotImplementedError):\n    mol_2d = Molecule.from_smiles(mol.to_smiles()).with_2d_coordinates()\n";

fn run_conversion(
    input: String,
    input_format: InputFormat,
    output_format: OutputFormat,
    mut result: Signal<Result<ConversionResult, String>>,
) {
    result.set(convert(&input, input_format, output_format));
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
pub fn FormatConverter() -> Element {
    let mut input_format = use_signal(|| InputFormat::Smiles);
    let mut output_format = use_signal(|| OutputFormat::SdfV2000);
    let mut input = use_signal(|| DEFAULT_SMILES.to_string());
    let mut uploaded_file = use_signal(|| None::<String>);
    let result =
        use_signal(|| convert(DEFAULT_SMILES, InputFormat::Smiles, OutputFormat::SdfV2000));
    let toast = use_context::<ToastManager>();
    let cosmolkit_version = cosmolkit::version();
    let python_code = python_example(&input(), input_format(), output_format());
    let can_swap = output_format().as_input().is_some()
        && output_for_input(input_format()).is_some()
        && result.read().is_ok();

    rsx! {
        Seo {
            title: "Molecular Format Converter — SDF, SMILES, MOL2, PDB | COSMolKit",
            description: "Convert SDF to SMILES, SMILES to SDF, MOL2 to PDB, PDB to SMILES, mmCIF and XYZ files, or molecular structures to SVG locally with COSMolKit.",
            canonical: "https://tools.cosmol.org/format-converter",
        }
        div { class: "min-h-screen uu-backdrop m-0 pt-[74px]",
            main { class: "mx-auto w-full max-w-6xl px-0 py-5 font-sans text-[#e8edf5] max-[800px]:px-3.5 max-[800px]:pb-[30px]",
                section { class: "w-full",
                    div { class: "mb-6 flex items-end justify-between gap-8 max-[800px]:flex-col max-[800px]:items-start max-[800px]:gap-4",
                        div {
                            Link { class: "text-[13px] font-semibold text-[#7ab5ff] no-underline hover:text-[#b4d6ff]", to: crate::route::Route::ToolDirectory {}, "Back to tools" }
                            h1 { class: "mb-1.5 mt-2.5 text-[32px] leading-[1.2] font-bold text-slate-50 max-[800px]:text-[27px]", "Molecular format converter" }
                            p { class: "m-0 max-w-[720px] text-[15px] leading-[1.6] text-[#9caabd]", "Convert between SMILES, SDF, MOL, MOL2, PDB, mmCIF, XYZ, and SVG locally while preserving the chemical graph and available coordinates." }
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

                    div { class: "overflow-hidden rounded-lg border border-[#213147] bg-[#0c1727] shadow-[0_20px_55px_rgba(0,0,0,0.24)]",
                        div { class: "grid grid-cols-[minmax(0,1fr)_54px_minmax(0,1fr)] border-b border-[#213147] max-[760px]:grid-cols-1",
                            div { class: "p-5 max-[480px]:p-4",
                                div { class: "mb-3 flex items-end justify-between gap-3",
                                    label { class: "block text-[13px] font-bold text-[#dce5f0]", r#for: "input-format", "Input format" }
                                    span { class: "text-[11px] text-[#718299]", "{input_format().detail()}" }
                                }
                                div { class: "relative",
                                    select {
                                        id: "input-format",
                                        class: "h-11 w-full cursor-pointer appearance-none rounded-md border border-[#2a3b52] bg-[#07111f] px-3 pr-12 text-[13px] font-semibold text-[#eaf1fa] outline-none hover:border-[#438ee9] focus:border-[#438ee9] focus:ring-[3px] focus:ring-[#3082ff1f]",
                                        value: input_format().id(),
                                        onchange: move |event| {
                                            let format = InputFormat::from_id(&event.value());
                                            let example = match example_input(format) {
                                                Ok(example) => example,
                                                Err(message) => {
                                                    let mut toast = toast;
                                                    toast.error(message);
                                                    return;
                                                }
                                            };
                                            input_format.set(format);
                                            input.set(example.clone());
                                            uploaded_file.set(None);
                                            run_conversion(example, format, output_format(), result);
                                        },
                                        for format in InputFormat::ALL {
                                            option { value: format.id(), "{format.label()}" }
                                        }
                                    }
                                    span { class: "pointer-events-none absolute top-1/2 right-4 -translate-y-1/2 text-[#c5d1df]",
                                        MdiIcon { size: 20, path: MDI_CHEVRON_DOWN }
                                    }
                                }
                            }

                            div { class: "grid place-items-center border-x border-[#213147] bg-[#091422] max-[760px]:h-12 max-[760px]:border-x-0 max-[760px]:border-y",
                                button {
                                    r#type: "button",
                                    class: if can_swap { "grid h-9 w-9 cursor-pointer place-items-center rounded-md border border-[#30435b] bg-[#101e30] text-lg text-[#9fc7f8] hover:border-[#438ee9] hover:text-white max-[760px]:rotate-90" } else { "grid h-9 w-9 cursor-not-allowed place-items-center rounded-md border border-[#26364a] bg-[#0b1726] text-lg text-[#526174] opacity-60 max-[760px]:rotate-90" },
                                    title: "Use the converted output as the new input",
                                    disabled: !can_swap,
                                    onclick: move |_| {
                                        let next_input = {
                                            let current = result.read();
                                            let Ok(current) = &*current else { return };
                                            current.content.clone()
                                        };
                                        let Some(next_input_format) = output_format().as_input() else { return };
                                        let Some(next_output_format) = output_for_input(input_format()) else { return };
                                        input_format.set(next_input_format);
                                        output_format.set(next_output_format);
                                        input.set(next_input.clone());
                                        uploaded_file.set(None);
                                        run_conversion(next_input, next_input_format, next_output_format, result);
                                    },
                                    "⇄"
                                }
                            }

                            div { class: "p-5 max-[480px]:p-4",
                                div { class: "mb-3 flex items-end justify-between gap-3",
                                    label { class: "block text-[13px] font-bold text-[#dce5f0]", r#for: "output-format", "Output format" }
                                    span { class: "text-[11px] text-[#718299]", ".{output_format().extension()}" }
                                }
                                div { class: "relative",
                                    select {
                                        id: "output-format",
                                        class: "h-11 w-full cursor-pointer appearance-none rounded-md border border-[#2a3b52] bg-[#07111f] px-3 pr-12 text-[13px] font-semibold text-[#eaf1fa] outline-none hover:border-[#438ee9] focus:border-[#438ee9] focus:ring-[3px] focus:ring-[#3082ff1f]",
                                        value: output_format().id(),
                                        onchange: move |event| {
                                            let format = OutputFormat::from_id(&event.value());
                                            output_format.set(format);
                                            run_conversion(input(), input_format(), format, result);
                                        },
                                        for format in OutputFormat::ALL {
                                            option { value: format.id(), "{format.label()}" }
                                        }
                                    }
                                    span { class: "pointer-events-none absolute top-1/2 right-4 -translate-y-1/2 text-[#c5d1df]",
                                        MdiIcon { size: 20, path: MDI_CHEVRON_DOWN }
                                    }
                                }
                            }
                        }

                        div { class: "grid min-h-[570px] grid-cols-2 max-[760px]:grid-cols-1",
                            section { class: "flex min-w-0 flex-col border-r border-[#213147] bg-[#0a1524] max-[760px]:border-r-0 max-[760px]:border-b",
                                div { class: "flex min-h-16 items-center justify-between gap-4 border-b border-[#213147] px-5 py-3 max-[480px]:px-4",
                                    div {
                                        h2 { class: "m-0 text-sm font-bold text-[#eef4fb]", "Source" }
                                        span { class: "mt-1 block text-[11px] text-[#718299]", "{input().len()} characters" }
                                    }
                                    label { class: "inline-flex h-9 cursor-pointer items-center justify-center rounded-[5px] border border-[#30435b] bg-[#0c1828] px-3 text-xs font-semibold text-[#c5d1df] hover:border-[#438ee9] hover:text-white",
                                        input {
                                            class: "hidden",
                                            r#type: "file",
                                            accept: input_format().accept(),
                                            onchange: move |event| {
                                                let files = event.files();
                                                async move {
                                                    let Some(file) = files.first() else { return };
                                                    let name = file.name();
                                                    match file.read_string().await {
                                                        Ok(content) if content.len() <= MAX_INPUT_LENGTH => {
                                                            uploaded_file.set(Some(name));
                                                            input.set(content.clone());
                                                            run_conversion(content, input_format(), output_format(), result);
                                                        }
                                                        Ok(_) => {
                                                            let mut toast = toast;
                                                            toast.error("File is too large (maximum 2 MB of text).");
                                                        }
                                                        Err(error) => {
                                                            let mut toast = toast;
                                                            toast.error(format!("Could not read file: {error}"));
                                                        }
                                                    }
                                                }
                                            },
                                        }
                                        "Choose file"
                                    }
                                }
                                if let Some(name) = &*uploaded_file.read() {
                                    div { class: "border-b border-[#213147] bg-[#0d1c2e] px-5 py-2 text-[11px] text-[#8fb7e8]", "Loaded: {name}" }
                                }
                                textarea {
                                    id: "converter-input",
                                    class: "min-h-[430px] flex-1 resize-y border-0 bg-[#07111f] p-5 font-mono text-[12px] leading-6 text-[#dce7f4] outline-none max-[480px]:min-h-[340px] max-[480px]:p-4",
                                    value: "{input}",
                                    maxlength: MAX_INPUT_LENGTH,
                                    spellcheck: false,
                                    oninput: move |event| {
                                        input.set(event.value());
                                        uploaded_file.set(None);
                                    },
                                }
                                div { class: "flex items-center justify-between gap-3 border-t border-[#213147] px-5 py-3 max-[480px]:px-4",
                                    button {
                                        r#type: "button",
                                        class: "cursor-pointer rounded-[5px] border border-[#30435b] bg-[#101e30] px-3 py-2 text-xs font-semibold text-[#b9c8da] hover:border-[#438ee9] hover:text-white",
                                        onclick: move |_| {
                                            let example = match example_input(input_format()) {
                                                Ok(example) => example,
                                                Err(message) => {
                                                    let mut toast = toast;
                                                    toast.error(message);
                                                    return;
                                                }
                                            };
                                            input.set(example.clone());
                                            uploaded_file.set(None);
                                            run_conversion(example, input_format(), output_format(), result);
                                        },
                                        "Load example"
                                    }
                                    button {
                                        r#type: "button",
                                        class: "h-9 cursor-pointer rounded-md border border-[#398cf6] bg-[#267de8] px-5 text-xs font-bold text-white shadow-[0_8px_24px_rgba(38,125,232,0.2)] hover:bg-[#358bf3] active:translate-y-px",
                                        onclick: move |_| run_conversion(input(), input_format(), output_format(), result),
                                        "Convert"
                                    }
                                }
                            }

                            section { class: "flex min-w-0 flex-col bg-[#111c2c]",
                                div { class: "flex min-h-16 items-center justify-between gap-4 border-b border-[#213147] px-5 py-3 max-[480px]:px-4",
                                    div {
                                        h2 { class: "m-0 text-sm font-bold text-[#eef4fb]", "Converted output" }
                                        if let Ok(output) = &*result.read() {
                                            span { class: "mt-1 block text-[11px] text-[#718299]", "{output.atom_count} atoms  /  {output.bond_count} bonds" }
                                        }
                                    }
                                    if let Ok(output) = &*result.read() {
                                        div { class: "flex gap-2",
                                            button {
                                                r#type: "button",
                                                class: "inline-flex h-9 cursor-pointer items-center justify-center rounded-[5px] border border-[#30435b] bg-[#0c1828] px-3 text-xs font-semibold text-[#c5d1df] hover:border-[#438ee9] hover:text-white",
                                                onclick: { let content = output.content.clone(); move |_| copy_text(content.clone(), "Converted output copied.", toast) },
                                                "Copy"
                                            }
                                            a {
                                                class: "inline-flex h-9 items-center justify-center rounded-[5px] border border-[#30435b] bg-[#0c1828] px-3 text-xs font-semibold text-[#c5d1df] no-underline hover:border-[#438ee9] hover:text-white",
                                                href: output.download_url.clone(),
                                                download: "molecule.{output_format().extension()}",
                                                "Download"
                                            }
                                        }
                                    }
                                }
                                match &*result.read() {
                                    Ok(output) => rsx! {
                                        if output_format() == OutputFormat::Svg {
                                            div { id: "converter-svg-preview", class: "grid min-h-[430px] flex-1 place-items-center bg-[#e9eef4] p-5 max-[480px]:min-h-[340px] max-[480px]:p-3",
                                                img { class: "max-h-full w-full object-contain", src: output.download_url.clone(), alt: "Converted molecular SVG" }
                                            }
                                        } else {
                                            textarea { id: "converter-output", class: "min-h-[430px] flex-1 resize-y border-0 bg-[#081321] p-5 font-mono text-[12px] leading-6 text-[#dce7f4] outline-none max-[480px]:min-h-[340px] max-[480px]:p-4", readonly: true, value: output.content.clone(), spellcheck: false }
                                        }
                                    },
                                    Err(error) => rsx! {
                                        div { class: "grid min-h-[430px] flex-1 place-items-center bg-[#0d1827] p-8 text-center max-[480px]:min-h-[340px]",
                                            div { class: "max-w-[430px]",
                                                div { class: "mx-auto mb-3 grid h-11 w-11 place-items-center rounded-full border border-[#713d46] bg-[#301820] text-lg font-bold text-[#f0a7b2]", "!" }
                                                h3 { class: "mb-2 mt-0 text-base font-bold text-white", "Conversion unavailable" }
                                                p { class: "m-0 text-[13px] leading-6 text-[#9caabd]", "{error}" }
                                            }
                                        }
                                    },
                                }
                                div { class: "border-t border-[#213147] px-5 py-3 text-[11px] text-[#718299]", "Converted locally. No structure data is uploaded." }
                            }
                        }
                    }
                }

                section { class: "mt-10 border-t border-[#213147] pt-8",
                    span { class: "text-xs font-bold text-[#4b96ff]", "SUPPORTED FORMATS" }
                    h2 { class: "mb-2 mt-2 text-xl font-bold text-slate-50", "Formats available in this converter" }
                    p { class: "m-0 max-w-[820px] text-sm leading-6 text-[#9caabd]",
                        "COSMolKit reads and writes the molecular graph locally. Coordinate-dependent exports use the coordinates available in the source or generated by the selected workflow."
                    }
                    div { class: "mt-5 grid grid-cols-2 gap-8 max-[700px]:grid-cols-1",
                        div {
                            h3 { class: "mb-2 mt-0 text-sm font-bold text-[#dce5f0]", "Input formats" }
                            p { class: "m-0 text-sm leading-6 text-[#9caabd]", "SMILES; MOL or SDF V2000/V3000; Tripos MOL2; PDB; PDBx/mmCIF; XYZ." }
                        }
                        div {
                            h3 { class: "mb-2 mt-0 text-sm font-bold text-[#dce5f0]", "Output formats" }
                            p { class: "m-0 text-sm leading-6 text-[#9caabd]", "SMILES; MOL V2000/V3000; SDF V2000/V3000; PDB; SVG depiction." }
                        }
                    }
                    div { class: "mt-7 border-t border-white/8 pt-6",
                        h3 { class: "mb-3 mt-0 text-base font-bold text-[#dce5f0]", "Common molecular file conversions" }
                        div { class: "grid grid-cols-2 gap-x-8 gap-y-4 max-[700px]:grid-cols-1",
                            p { class: "m-0 text-sm leading-6 text-[#9caabd]",
                                strong { class: "font-semibold text-[#cbd7e5]", "Convert files to SMILES: " }
                                "SDF to SMILES, MOL to SMILES, MOL2 to SMILES, PDB to SMILES, mmCIF to SMILES, and XYZ to SMILES."
                            }
                            p { class: "m-0 text-sm leading-6 text-[#9caabd]",
                                strong { class: "font-semibold text-[#cbd7e5]", "Export from SMILES: " }
                                "SMILES to SDF V2000/V3000, SMILES to MOL V2000/V3000, SMILES to PDB, and SMILES to SVG."
                            }
                            p { class: "m-0 text-sm leading-6 text-[#9caabd]",
                                strong { class: "font-semibold text-[#cbd7e5]", "Convert molecular structure files: " }
                                "MOL2 to SDF, MOL2 to PDB, SDF to MOL, SDF to PDB, PDB to SDF, mmCIF to PDB, and XYZ to SDF."
                            }
                            p { class: "m-0 text-sm leading-6 text-[#9caabd]",
                                strong { class: "font-semibold text-[#cbd7e5]", "Create a 2D structure image: " }
                                "Render SMILES, MOL, SDF, MOL2, PDB, mmCIF, or XYZ input as a scalable SVG molecular depiction."
                            }
                        }
                    }
                }

                section { class: "mt-10 border-t border-[#213147] pt-8",
                    div { class: "flex items-start justify-between gap-6 max-[800px]:flex-col",
                        div {
                            span { class: "text-xs font-bold text-[#4b96ff]", "PYTHON BACKEND" }
                            h2 { class: "mb-1.5 mt-2 text-xl font-bold text-slate-50", "Run the same conversion with COSMolKit" }
                            p { class: "m-0 max-w-[680px] text-sm leading-6 text-[#9caabd]", "The example tracks the formats and source currently selected above." }
                        }
                        a { class: "inline-flex shrink-0 items-center gap-2 rounded-md border border-[#2a3b52] bg-[#0c1828] px-3 py-2 text-xs font-semibold text-[#b8c5d6] no-underline hover:border-[#438ee9] hover:text-white", href: "https://pypi.org/project/cosmolkit/{cosmolkit_version}/", target: "_blank", rel: "noreferrer", "COSMolKit Python {cosmolkit_version}" MdiIcon { size: 14, path: MDI_OPEN_IN_NEW } }
                    }
                    div { class: "mt-6 grid grid-cols-[240px_minmax(0,1fr)] gap-6 max-[800px]:grid-cols-1",
                        div { class: "border-l-2 border-[#267de8] pl-4",
                            span { class: "block text-[11px] font-bold text-[#718299]", "INSTALL" }
                            code { class: "mt-2 block break-all font-mono text-[13px] text-[#dce5f0]", "pip install cosmolkit=={cosmolkit_version}" }
                            p { class: "mb-0 mt-3 text-xs leading-5 text-[#718299]", "Python 3.9+ / Rust-native wheel" }
                        }
                        div { class: "min-w-0 overflow-hidden rounded-lg border border-[#213147] bg-[#081321]",
                            div { class: "flex min-h-11 items-center justify-between border-b border-[#213147] px-4",
                                div { class: "flex items-center gap-2", span { class: "h-2 w-2 rounded-full bg-[#f0c35a]" } span { class: "text-xs font-semibold text-[#9caabd]", "convert_molecule.py" } }
                                button { r#type: "button", class: "cursor-pointer rounded-[5px] border border-[#30435b] bg-[#0c1828] px-2.5 py-1.5 text-[11px] font-semibold text-[#c5d1df] hover:border-[#438ee9] hover:text-white", onclick: { let code = python_code.clone(); move |_| copy_text(code.clone(), "Python example copied.", toast) }, "Copy Python" }
                            }
                            pre { class: "m-0 max-h-[430px] overflow-auto p-4 font-mono text-[13px] leading-6 text-[#d6e2f0]", code { "{python_code}" } }
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
    fn converts_smiles_to_v2000_mol() {
        let output = convert("CCO", InputFormat::Smiles, OutputFormat::MolV2000).unwrap();
        assert!(output.content.contains("V2000"));
        assert_eq!(output.atom_count, 3);
        assert_eq!(output.bond_count, 2);
    }

    #[test]
    fn mol_roundtrips_to_smiles() {
        let mol = convert("CCO", InputFormat::Smiles, OutputFormat::MolV2000).unwrap();
        let smiles = convert(&mol.content, InputFormat::MolSdf, OutputFormat::Smiles).unwrap();
        assert_eq!(smiles.content, "CCO");
    }

    #[test]
    fn rejects_invalid_input() {
        assert!(convert("C(", InputFormat::Smiles, OutputFormat::Smiles).is_err());
    }

    #[test]
    fn output_metadata_matches_format() {
        assert_eq!(OutputFormat::SdfV3000.extension(), "sdf");
        assert_eq!(OutputFormat::Svg.mime(), "image/svg+xml");
        let url = data_url(OutputFormat::Pdb.mime(), "ATOM\n");
        assert!(url.starts_with("data:chemical/x-pdb;charset=utf-8,"));
    }

    #[test]
    fn every_input_format_has_a_parseable_example() {
        for format in InputFormat::ALL {
            let input = example_input(format).expect("example generation should succeed");
            parse_molecule(&input, format)
                .unwrap_or_else(|error| panic!("{} example failed: {error}", format.label()));
        }
    }

    #[test]
    fn python_example_preserves_structured_file_headers() {
        let mol = example_input(InputFormat::MolSdf).expect("MOL example should generate");
        assert!(mol.starts_with('\n'));
        let example = python_example(&mol, InputFormat::MolSdf, OutputFormat::Smiles);
        assert!(example.contains("source = \"\\n  COSMolKit"));
    }

    #[test]
    fn python_example_prepares_molecules_for_2d_outputs() {
        for output in [
            OutputFormat::MolV2000,
            OutputFormat::MolV3000,
            OutputFormat::SdfV2000,
            OutputFormat::SdfV3000,
            OutputFormat::Svg,
        ] {
            let input = example_input(InputFormat::Xyz).expect("XYZ example should generate");
            let example = python_example(&input, InputFormat::Xyz, output);
            assert!(example.contains("mol.with_2d_coordinates()"));
            assert!(example.contains("Molecule.from_smiles(mol.to_smiles())"));
        }

        let input = example_input(InputFormat::Smiles).expect("SMILES example should generate");
        let pdb = python_example(&input, InputFormat::Smiles, OutputFormat::Pdb);
        assert!(!pdb.contains("mol.with_2d_coordinates()"));
    }
}
