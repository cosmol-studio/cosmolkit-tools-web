"""Regression test for the Python examples displayed by the web tools."""

from __future__ import annotations

import re
import sys
import tempfile
from pathlib import Path
from typing import Callable

import cosmolkit
from cosmolkit import Molecule


ROOT = Path(__file__).resolve().parents[1]

SOURCES = {
    "smiles": "CCO",
    "mol_sdf": Molecule.from_smiles("CCO")
    .to_2d_sdf_string(format="v2000")
    .removesuffix("$$$$\n"),
    "mol2": """@<TRIPOS>MOLECULE
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
""",
    "pdb": """HETATM    1  O   HOH A   1       0.000   0.000   0.000  1.00 20.00           O
HETATM    2  H1  HOH A   1       0.957   0.000   0.000  1.00 20.00           H
HETATM    3  H2  HOH A   1      -0.240   0.927   0.000  1.00 20.00           H
CONECT    1    2    3
END
""",
    "mmcif": """data_water
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
""",
    "xyz": """3
water
O 0.000 0.000 0.000
H 0.957 0.000 0.000
H -0.240 0.927 0.000
""",
}

CONSTRUCTORS: dict[str, Callable[[str], Molecule]] = {
    "smiles": Molecule.from_smiles,
    "mol_sdf": Molecule.read_mol_from_str,
    "mol2": Molecule.read_mol2_from_str,
    "pdb": Molecule.from_pdb_block,
    "mmcif": Molecule.from_mmcif_block,
    "xyz": Molecule.from_xyz_block,
}

EXTENSIONS = {
    "smiles": "smi",
    "mol_v2000": "mol",
    "mol_v3000": "mol",
    "sdf_v2000": "sdf",
    "sdf_v3000": "sdf",
    "pdb": "pdb",
    "svg": "svg",
}


def expected_cosmolkit_version() -> str:
    cargo_toml = (ROOT / "Cargo.toml").read_text(encoding="utf-8")
    match = re.search(r'^cosmolkit\s*=\s*"([^"]+)"', cargo_toml, re.MULTILINE)
    if match is None:
        raise AssertionError("Could not find the cosmolkit dependency in Cargo.toml")
    return match.group(1)


def prepare_2d(molecule: Molecule) -> Molecule:
    try:
        return molecule.with_2d_coordinates()
    except (ValueError, NotImplementedError):
        return Molecule.from_smiles(molecule.to_smiles()).with_2d_coordinates()


def writers() -> dict[str, Callable[[Molecule], str]]:
    return {
        "smiles": lambda molecule: molecule.to_smiles(),
        "mol_v2000": lambda molecule: prepare_2d(molecule)
        .to_2d_sdf_string(format="v2000")
        .removesuffix("$$$$\n"),
        "mol_v3000": lambda molecule: prepare_2d(molecule)
        .to_2d_sdf_string(format="v3000")
        .removesuffix("$$$$\n"),
        "sdf_v2000": lambda molecule: prepare_2d(molecule).to_2d_sdf_string(
            format="v2000"
        ),
        "sdf_v3000": lambda molecule: prepare_2d(molecule).to_2d_sdf_string(
            format="v3000"
        ),
        "pdb": lambda molecule: molecule.to_pdb_block(),
        "svg": lambda molecule: prepare_2d(molecule).to_svg(width=720, height=480),
    }


def validate_output(output_format: str, output: str) -> None:
    if not output:
        raise AssertionError(f"{output_format} output is empty")
    if output_format == "svg" and "<svg" not in output:
        raise AssertionError("SVG marker is missing")
    if output_format.startswith("sdf_") and "$$$$" not in output:
        raise AssertionError("SDF record delimiter is missing")
    if output_format == "mol_v2000" and "V2000" not in output:
        raise AssertionError("V2000 marker is missing")
    if output_format == "mol_v3000" and "V3000" not in output:
        raise AssertionError("V3000 marker is missing")


def test_format_converter(directory: Path) -> int:
    completed = 0
    output_writers = writers()
    for input_format, source in SOURCES.items():
        molecule = CONSTRUCTORS[input_format](source)
        for output_format, writer in output_writers.items():
            output = writer(molecule)
            validate_output(output_format, output)
            path = directory / f"{input_format}-{output_format}.{EXTENSIONS[output_format]}"
            path.write_text(output, encoding="utf-8")
            if path.read_text(encoding="utf-8") != output:
                raise AssertionError(f"File round trip failed for {path.name}")
            completed += 1
        print(f"{input_format}: {len(output_writers)}/{len(output_writers)} outputs passed")
    return completed


def test_smiles_to_svg(directory: Path) -> None:
    smiles = "Cn1c(=O)c2c(ncn2C)n(C)c1=O"
    molecule = Molecule.from_smiles(smiles).with_2d_coordinates()
    svg = molecule.to_svg(width=720, height=480)
    validate_output("svg", svg)
    path = directory / "molecule.svg"
    path.write_text(svg, encoding="utf-8")
    if path.read_text(encoding="utf-8") != svg:
        raise AssertionError("SMILES to SVG file round trip failed")


def main() -> None:
    expected_version = expected_cosmolkit_version()
    installed_version = cosmolkit.__version__
    if installed_version != expected_version:
        raise AssertionError(
            f"COSMolKit version mismatch: expected {expected_version}, got {installed_version}"
        )

    print(f"python={sys.executable}")
    print(f"cosmolkit={installed_version}")
    with tempfile.TemporaryDirectory(prefix="cosmolkit-python-examples-") as temp_dir:
        directory = Path(temp_dir)
        completed = test_format_converter(directory)
        test_smiles_to_svg(directory)

    print(f"format converter: {completed}/42 combinations passed")
    print("smiles to SVG file example: passed")


if __name__ == "__main__":
    main()
