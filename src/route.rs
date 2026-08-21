use dioxus::prelude::*;

use crate::{
    component::Navbar,
    page::{
        CheckPains, ConformerGenerator, Ecosystem, FormatConverter, Home, InchiTool,
        MolecularProperties, SmilesCanonicalizer, SmilesToSvg, ToolDirectory,
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
}

#[cfg(all(feature = "ssg", not(target_arch = "wasm32")))]
#[server(endpoint = "static_routes", output = server_fn::codec::Json)]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    Ok(Route::static_routes()
        .iter()
        .map(ToString::to_string)
        .collect())
}
