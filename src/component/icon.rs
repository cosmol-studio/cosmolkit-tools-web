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
            path { d: "M24 6 39.6 15v18L24 42 8.4 33V15Z" }
            path { d: "M24 11.2 35.1 17.6M35.1 30.4 24 36.8M12.9 30.4V17.6" }
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
            path { d: "M28 6H13a3 3 0 0 0-3 3v30a3 3 0 0 0 3 3h22a3 3 0 0 0 3-3V16Z" }
            path { d: "M28 6v7a3 3 0 0 0 3 3h7" }
            path { d: "M16 24h16m-4-4 4 4-4 4M32 34H16m4-4-4 4 4 4" }
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
            path { d: "M20.2 26.4 12.7 32.6M27.8 26.4 35.3 32.6M25.8 18.3 28.7 11.7" }
            path { d: "M20.8 19.2 15.3 12.7" }
            circle { cx: "24", cy: "23", r: "5", fill: "currentColor", fill_opacity: "0.12" }
            circle { cx: "9.5", cy: "35.2", r: "4" }
            circle { cx: "38.5", cy: "35.2", r: "4" }
            circle { cx: "30.3", cy: "8", r: "4" }
            circle { cx: "13", cy: "10", r: "3.5" }
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
            path { d: "M25 20a10 10 0 1 0-6 6l15 15h7v-7h-5v-5h-5Z" }
            circle { cx: "13", cy: "14", r: "2.5" }
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
            rect { x: "10", y: "6", width: "28", height: "36", rx: "4" }
            rect { x: "16", y: "12", width: "16", height: "8", rx: "1" }
            path { d: "M16 27h2m6 0h2m-10 8h2m6 0h2M32 27v8" }
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
            circle { cx: "10", cy: "10", r: "3.5" }
            circle { cx: "10", cy: "38", r: "3.5" }
            path { d: "M10 13.5V16a8 8 0 0 0 8 8h13M10 34.5V32a8 8 0 0 1 8-8M27 20l4 4-4 4" }
            circle { cx: "40", cy: "24", r: "4" }
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
            path { d: "M6 8h30v4L24 25v13l-6 3V25L6 12Z" }
            circle { cx: "36", cy: "34", r: "8", fill: "currentColor", fill_opacity: "0.08" }
            path { d: "M36 30v4" }
            circle { cx: "36", cy: "38", r: "1", fill: "currentColor", stroke: "none" }
        }
    }
}
