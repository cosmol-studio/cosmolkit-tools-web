use dioxus::prelude::*;

use crate::{
    component::Navbar,
    page::{
        CheckPains, ConformerGenerator, FormatConverter, Home, InchiTool, SmilesToSvg,
        ToolDirectory,
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
    #[route("/tools/smiles-to-svg")]
    SmilesToSvg {},
    #[route("/tools/format-converter")]
    FormatConverter {},
    #[route("/tools/conformer-generator")]
    ConformerGenerator {},
    #[route("/tools/inchi")]
    InchiTool {},
    #[route("/tools/check-pains")]
    CheckPains {},
}
