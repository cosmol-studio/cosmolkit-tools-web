use dioxus::prelude::*;

use crate::{
    component::Navbar,
    page::{
        Blog, CheckPains, ConformerGenerator, ConversionSlug, Ecosystem, FormatConversion,
        FormatConverter, Home, InchiTool, MolecularProperties, RdkitAlternativeRust,
        RustCheminformatics, RustCheminformaticsLibraries, SmilesCanonicalizer, SmilesToSvg,
        ToolDirectory, Validation,
    },
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/tools")]
    ToolDirectory {},
    #[route("/ecosystem")]
    Ecosystem {},
    #[route("/blog")]
    Blog {},
    #[route("/rust-cheminformatics")]
    RustCheminformatics {},
    #[route("/rdkit-alternative-rust")]
    RdkitAlternativeRust {},
    #[route("/rust-cheminformatics-libraries")]
    RustCheminformaticsLibraries {},
    #[route("/validation")]
    Validation {},
    #[route("/smiles-to-svg")]
    SmilesToSvg {},
    #[route("/format-converter")]
    FormatConverter {},
    #[route("/conformer-generator")]
    ConformerGenerator {},
    #[route("/inchi")]
    InchiTool {},
    #[route("/molecular-properties")]
    MolecularProperties {},
    #[route("/smiles-canonicalizer")]
    SmilesCanonicalizer {},
    #[route("/check-pains")]
    CheckPains {},
    #[route("/:conversion")]
    FormatConversion { conversion: ConversionSlug },
}

#[cfg(all(feature = "ssg", not(target_arch = "wasm32")))]
#[server(endpoint = "static_routes", output = server_fn::codec::Json)]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    let mut routes: Vec<String> = Route::static_routes()
        .iter()
        .map(ToString::to_string)
        .collect();
    routes.extend(
        ConversionSlug::all()
            .into_iter()
            .map(|conversion| Route::FormatConversion { conversion }.to_string()),
    );
    Ok(routes)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn conversion_slugs_do_not_shadow_existing_routes() {
        assert_eq!(Route::from_str("/inchi").unwrap(), Route::InchiTool {});
        assert_eq!(
            Route::from_str("/sdf-to-smiles").unwrap(),
            Route::FormatConversion {
                conversion: "sdf-to-smiles".parse().unwrap(),
            }
        );
        assert_eq!(
            Route::from_str("/smiles-to-svg").unwrap(),
            Route::SmilesToSvg {}
        );
        assert!(Route::from_str("/smiles-to-svg-converter").is_err());
        assert!(Route::from_str("/pdb-to-svg").is_err());
        assert!(Route::from_str("/not-a-supported-conversion").is_err());
    }
}
