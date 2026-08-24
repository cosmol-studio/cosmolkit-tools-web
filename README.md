# COSMolKit Tools

Browser-based cheminformatics and molecular visualization tools powered by Rust and WebAssembly.

**Use the tools at [tools.cosmol.org](https://tools.cosmol.org/).** Molecular structures are processed locally in the browser and are not uploaded to a backend service.

## Available tools

| Tool | What it does |
| --- | --- |
| [SMILES to SVG](https://tools.cosmol.org/smiles-to-svg) | Parse SMILES, generate 2D coordinates, and export a scalable SVG molecular depiction. |
| [Molecular format converter](https://tools.cosmol.org/format-converter) | Read SMILES, MOL/SDF V2000 and V3000, MOL2, PDB, mmCIF, or XYZ, then export SMILES, MOL/SDF, PDB, or SVG. |
| [3D conformer generator](https://tools.cosmol.org/conformer-generator) | Generate a conformer from SMILES with ETKDG v2, ETKDG v3, or KDG, preview it in 3D, and export SDF V3000 or PDB coordinates. |
| [InChI converter](https://tools.cosmol.org/inchi) | Convert SMILES to standard InChI and InChIKey, or parse InChI back into a molecular structure and canonical SMILES. |
| [Molecular properties](https://tools.cosmol.org/molecular-properties) | Calculate formula, molecular weight, exact mass, heavy atoms, HBD, HBA, TPSA, rotatable bonds, logP, and formal charge from SMILES. |
| [SMILES canonicalizer](https://tools.cosmol.org/smiles-canonicalizer) | Generate canonical, isomeric, and kekulized SMILES and inspect hydrogen count and formal charge. |

Browse the complete directory at [tools.cosmol.org/tools](https://tools.cosmol.org/tools).

## Built on the COSMol ecosystem

COSMolKit Tools brings together three focused open-source Rust projects:

- [COSMolKit](https://github.com/cosmol-studio/COSMolKit) provides molecular graphs, cheminformatics algorithms, format readers and writers, descriptors, coordinates, SMILES, and InChI.
- [COSMol-viewer](https://github.com/cosmol-studio/COSMol-viewer) provides interactive molecular and structural biology visualization, including the 3D conformer preview.
- [cosmolkit-tools-web](https://github.com/cosmol-studio/cosmolkit-tools-web) combines the chemistry core and viewer into accessible browser workflows.

The interface and application logic are written in Rust with [Dioxus](https://dioxuslabs.com/) and compiled to WebAssembly. There is no hand-written JavaScript or TypeScript glue layer, and the tools do not require an application backend.

## Documentation

- [COSMolKit documentation](https://kit.cosmol.org/)
- [COSMolKit on crates.io](https://crates.io/crates/cosmolkit)
- [COSMol-viewer documentation](https://cosmol-studio.github.io/COSMol-viewer/)
- [COSMol ecosystem overview](https://tools.cosmol.org/ecosystem)

Development and production build instructions are documented in [dev.md](dev.md).

## License

This project is available under the [MIT License](LICENSE).
