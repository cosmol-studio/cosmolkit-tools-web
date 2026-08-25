use std::{fmt, str::FromStr};

use dioxus::prelude::*;

use super::format_converter::{FormatConverter, FormatConverterPreset};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ConversionInput {
    Smiles,
    Mol,
    Sdf,
    Mol2,
    Pdb,
    Mmcif,
    Xyz,
}

impl ConversionInput {
    const ALL: [Self; 7] = [
        Self::Smiles,
        Self::Mol,
        Self::Sdf,
        Self::Mol2,
        Self::Pdb,
        Self::Mmcif,
        Self::Xyz,
    ];

    fn slug(self) -> &'static str {
        match self {
            Self::Smiles => "smiles",
            Self::Mol => "mol",
            Self::Sdf => "sdf",
            Self::Mol2 => "mol2",
            Self::Pdb => "pdb",
            Self::Mmcif => "mmcif",
            Self::Xyz => "xyz",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Smiles => "SMILES",
            Self::Mol => "MOL",
            Self::Sdf => "SDF",
            Self::Mol2 => "MOL2",
            Self::Pdb => "PDB",
            Self::Mmcif => "mmCIF",
            Self::Xyz => "XYZ",
        }
    }

    fn format_id(self) -> &'static str {
        match self {
            Self::Smiles => "smiles",
            Self::Mol => "mol",
            Self::Sdf => "sdf",
            Self::Mol2 => "mol2",
            Self::Pdb => "pdb",
            Self::Mmcif => "mmcif",
            Self::Xyz => "xyz",
        }
    }

    fn parse(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|format| format.slug() == value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ConversionOutput {
    Smiles,
    MolV2000,
    MolV3000,
    SdfV2000,
    SdfV3000,
    Pdb,
    Svg,
}

impl ConversionOutput {
    const ALL: [Self; 7] = [
        Self::Smiles,
        Self::MolV2000,
        Self::MolV3000,
        Self::SdfV2000,
        Self::SdfV3000,
        Self::Pdb,
        Self::Svg,
    ];

    fn slug(self) -> &'static str {
        match self {
            Self::Smiles => "smiles",
            Self::MolV2000 => "mol",
            Self::MolV3000 => "mol-v3000",
            Self::SdfV2000 => "sdf",
            Self::SdfV3000 => "sdf-v3000",
            Self::Pdb => "pdb",
            Self::Svg => "svg",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Smiles => "SMILES",
            Self::MolV2000 => "MOL",
            Self::MolV3000 => "MOL V3000",
            Self::SdfV2000 => "SDF",
            Self::SdfV3000 => "SDF V3000",
            Self::Pdb => "PDB",
            Self::Svg => "SVG",
        }
    }

    fn format_id(self) -> &'static str {
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

    fn parse(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|format| format.slug() == value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ConversionSlug {
    SmilesConverter,
    Pair {
        input: ConversionInput,
        output: ConversionOutput,
    },
}

impl ConversionSlug {
    #[cfg_attr(
        not(all(feature = "ssg", not(target_arch = "wasm32"))),
        allow(dead_code)
    )]
    pub fn all() -> Vec<Self> {
        let mut routes = vec![Self::SmilesConverter];
        for input in ConversionInput::ALL {
            for output in ConversionOutput::ALL {
                let conversion = Self::Pair { input, output };
                if conversion.has_dedicated_route() && conversion.to_string() != "smiles-to-svg" {
                    routes.push(conversion);
                }
            }
        }
        routes
    }

    pub(crate) fn from_format_ids(input_slug: &str, output_id: &str) -> Option<Self> {
        let input = ConversionInput::parse(input_slug)?;
        let output = ConversionOutput::ALL
            .into_iter()
            .find(|format| format.format_id() == output_id)?;
        let conversion = Self::Pair { input, output };
        conversion.has_dedicated_route().then_some(conversion)
    }

    fn has_dedicated_route(self) -> bool {
        let Self::Pair { input, output } = self else {
            return true;
        };

        let supported = output != ConversionOutput::Svg
            || matches!(
                input,
                ConversionInput::Smiles | ConversionInput::Mol | ConversionInput::Sdf
            );
        let rewrites_same_format = matches!(
            (input, output),
            (ConversionInput::Smiles, ConversionOutput::Smiles)
                | (
                    ConversionInput::Mol,
                    ConversionOutput::MolV2000 | ConversionOutput::MolV3000
                )
                | (
                    ConversionInput::Sdf,
                    ConversionOutput::SdfV2000 | ConversionOutput::SdfV3000
                )
                | (ConversionInput::Pdb, ConversionOutput::Pdb)
        );

        supported && !rewrites_same_format
    }

    pub(crate) fn preset(self) -> FormatConverterPreset {
        match self {
            Self::SmilesConverter => FormatConverterPreset::new(
                "SMILES Converter Online — Powered by Rust | COSMolKit",
                "Convert SMILES to SDF, MOL, PDB, or SVG online with COSMolKit. Browser-native molecular conversion powered by Rust and WebAssembly with no structure upload.",
                "https://tools.cosmol.org/smiles-converter",
                "SMILES converter online",
                "Convert SMILES to SDF, MOL, PDB, or SVG locally in your browser with COSMolKit, Rust, and WebAssembly.",
                "smiles",
                "sdf-v2000",
            ),
            Self::Pair {
                input: ConversionInput::Smiles,
                output: ConversionOutput::Svg,
            } => FormatConverterPreset::new(
                "SMILES to SVG — Molecular Structure Renderer | COSMolKit",
                "Free browser SMILES renderer and chemical structure drawing tool. Convert SMILES to a scalable SVG molecule image locally with COSMolKit and WebAssembly.",
                "https://tools.cosmol.org/smiles-to-svg",
                "SMILES to SVG",
                "Generate a scalable 2D molecular structure from SMILES locally in your browser.",
                "smiles",
                "svg",
            ),
            Self::Pair { input, output } => {
                let input_label = input.label();
                let output_label = output.label();
                let slug = self.to_string();
                FormatConverterPreset::new(
                    &format!(
                        "{input_label} to {output_label} Converter Online — Powered by Rust | COSMolKit"
                    ),
                    &format!(
                        "Convert {input_label} to {output_label} online with COSMolKit. The browser-native Rust and WebAssembly workflow processes molecular data locally without an upload."
                    ),
                    &format!("https://tools.cosmol.org/{slug}"),
                    &format!("{input_label} to {output_label} converter"),
                    &format!(
                        "Convert {input_label} to {output_label} locally in your browser with COSMolKit, Rust, and WebAssembly."
                    ),
                    input.format_id(),
                    output.format_id(),
                )
                .with_input_slug(input.slug())
            }
        }
    }
}

impl fmt::Display for ConversionSlug {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SmilesConverter => formatter.write_str("smiles-converter"),
            Self::Pair { input, output } => {
                write!(formatter, "{}-to-{}", input.slug(), output.slug())
            }
        }
    }
}

impl FromStr for ConversionSlug {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value == "smiles-converter" {
            return Ok(Self::SmilesConverter);
        }
        let (input, output) = value
            .split_once("-to-")
            .ok_or("unsupported molecular conversion route")?;
        let conversion = Self::Pair {
            input: ConversionInput::parse(input).ok_or("unsupported molecular conversion input")?,
            output: ConversionOutput::parse(output)
                .ok_or("unsupported molecular conversion output")?,
        };
        conversion
            .has_dedicated_route()
            .then_some(conversion)
            .ok_or("unsupported molecular conversion route")
    }
}

#[component]
pub fn FormatConversion(conversion: ConversionSlug) -> Element {
    rsx! { FormatConverter { preset: conversion.preset() } }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_shareable_conversion_route_round_trips() {
        let routes = ConversionSlug::all();
        assert_eq!(routes.len(), 39);
        for conversion in routes {
            let slug = conversion.to_string();
            assert_eq!(slug.parse::<ConversionSlug>().unwrap(), conversion);
        }
    }

    #[test]
    fn unsupported_combinations_do_not_become_seo_routes() {
        assert!(ConversionSlug::from_format_ids("pdb", "svg").is_none());
        assert!(ConversionSlug::from_format_ids("mol2", "svg").is_none());
        assert!(ConversionSlug::from_format_ids("mmcif", "svg").is_none());
        assert!(ConversionSlug::from_format_ids("xyz", "svg").is_none());
        assert!(ConversionSlug::from_format_ids("smiles", "smiles").is_none());
        assert!(ConversionSlug::from_format_ids("smiles", "sdf-v3000").is_some());
        assert!("pdb-to-svg".parse::<ConversionSlug>().is_err());
        assert!("smiles-to-sdf-v3000".parse::<ConversionSlug>().is_ok());
    }

    #[test]
    fn smiles_to_svg_reuses_the_existing_tool_route() {
        let conversion = ConversionSlug::from_format_ids("smiles", "svg").unwrap();
        assert_eq!(conversion.to_string(), "smiles-to-svg");
        assert_eq!(
            conversion.preset().canonical,
            "https://tools.cosmol.org/smiles-to-svg"
        );
    }
}
