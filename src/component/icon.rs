use dioxus::prelude::*;

pub const MDI_OPEN_IN_NEW: &str = r#"M14,3V5H17.59L7.76,14.83L9.17,16.24L19,6.41V10H21V3M19,19H5V5H12V3H5C3.89,3 3,3.9 3,5V19A2,2 0 0,0 5,21H19A2,2 0 0,0 21,19V12H19V19Z"#;
pub const MDI_CHEVRON_DOWN: &str = r#"M7.41,8.58L12,13.17L16.59,8.58L18,10L12,16L6,10L7.41,8.58Z"#;

#[component]
pub fn MdiIcon(
    path: &'static str,
    #[props(default = 24)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
    #[props(default = "")] class: &'static str,
) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "{color}",
            class: "{class}",
            path {
                d: "{path}"
            }
        }
    }
}

#[component]
pub fn DepictionCardIcon(class: &'static str, label: &'static str) -> Element {
    rsx! {
        svg {
            class,
            view_box: "0 0 48 48",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2.2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            role: "img",
            "aria-label": label,
            "data-card-icon": "depiction",
            path { d: "M16 10h16l8 14-8 14H16L8 24l8-14Z" }
            path { d: "M18 14h12M35.5 24l-6 10M18 34l-5.5-10" }
        }
    }
}

#[component]
pub fn FormatCardIcon(class: &'static str, label: &'static str) -> Element {
    rsx! {
        svg {
            class,
            view_box: "0 0 48 48",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2.2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            role: "img",
            "aria-label": label,
            "data-card-icon": "format",
            path { d: "M13 5h15l8 8v30H13V5Z" }
            path { d: "M28 5v8h8" }
            path { d: "M18 23h13l-3-3M31 23l-3 3M31 33H18l3-3M18 33l3 3" }
        }
    }
}

#[component]
pub fn ConformerCardIcon(class: &'static str, label: &'static str) -> Element {
    rsx! {
        svg {
            class,
            view_box: "0 0 48 48",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2.2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            role: "img",
            "aria-label": label,
            "data-card-icon": "conformer",
            path { d: "M12 34l11-10M23 24l13 8M23 24l5-13" }
            path { d: "M15 12l8 12", stroke_dasharray: "2.5 3", opacity: "0.7" }
            circle { cx: "10", cy: "36", r: "4" }
            circle { cx: "23", cy: "24", r: "4.5" }
            circle { cx: "38", cy: "33", r: "4" }
            circle { cx: "29", cy: "9", r: "3.5" }
            circle { cx: "13", cy: "9", r: "3" }
        }
    }
}

#[component]
pub fn IdentifierCardIcon(class: &'static str, label: &'static str) -> Element {
    rsx! {
        svg {
            class,
            view_box: "0 0 48 48",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2.2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            role: "img",
            "aria-label": label,
            "data-card-icon": "identifier",
            circle { cx: "16", cy: "20", r: "8" }
            circle { cx: "16", cy: "20", r: "2.5" }
            path { d: "M23 24l14 14M31 32l4-4M35 36l4-4" }
            path { d: "M9 38h14M9 42h21", opacity: "0.65" }
        }
    }
}

#[component]
pub fn PropertiesCardIcon(class: &'static str, label: &'static str) -> Element {
    rsx! {
        svg {
            class,
            view_box: "0 0 48 48",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2.2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            role: "img",
            "aria-label": label,
            "data-card-icon": "properties",
            rect { x: "9", y: "6", width: "30", height: "36", rx: "3" }
            path { d: "M15 12h18v7H15zM16 35v-5M24 35V25M32 35V22" }
            circle { cx: "16", cy: "35", r: "1.5", fill: "currentColor", stroke: "none" }
            circle { cx: "24", cy: "25", r: "1.5", fill: "currentColor", stroke: "none" }
            circle { cx: "32", cy: "22", r: "1.5", fill: "currentColor", stroke: "none" }
        }
    }
}

#[component]
pub fn CanonicalCardIcon(class: &'static str, label: &'static str) -> Element {
    rsx! {
        svg {
            class,
            view_box: "0 0 48 48",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2.2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            role: "img",
            "aria-label": label,
            "data-card-icon": "canonical",
            path { d: "M8 14h8l5 6 6-12 6 6h7" }
            circle { cx: "8", cy: "14", r: "2.5" }
            circle { cx: "40", cy: "14", r: "2.5" }
            path { d: "M18 25l6 5 6-5M24 29v7" }
            path { d: "M10 39h28" }
            circle { cx: "10", cy: "39", r: "2.5" }
            circle { cx: "38", cy: "39", r: "2.5" }
        }
    }
}

#[component]
pub fn FilterAlertCardIcon(class: &'static str, label: &'static str) -> Element {
    rsx! {
        svg {
            class,
            view_box: "0 0 48 48",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2.2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            role: "img",
            "aria-label": label,
            "data-card-icon": "filter-alert",
            path { d: "M7 9h34L28 25v13l-8 4V25L7 9Z" }
            circle { cx: "37", cy: "35", r: "7", fill: "currentColor", fill_opacity: "0.08" }
            path { d: "M37 31v5M37 39h.01" }
        }
    }
}
