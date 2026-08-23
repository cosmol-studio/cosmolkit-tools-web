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
pub fn MoleculeCardIcon(class: &'static str, label: &'static str) -> Element {
    rsx! {
        svg {
            class,
            view_box: "0 0 194 168",
            fill: "none",
            stroke: "#185E91",
            stroke_width: "7",
            stroke_linejoin: "round",
            role: "img",
            "aria-label": label,
            "data-card-icon": "molecule",
            polygon { points: "50.7 3.5 143.5 3.8 189.6 84.3 143 164.6 50.2 164.3 4 83.8 50.7 3.5" }
            path { d: "M58.6 17.4 135.4 17.7" }
            path { d: "m173.6 84.3-38.6 66.3" }
            path { d: "m58.2 150.3-38.1-66.5" }
        }
    }
}

#[component]
pub fn SdfCardIcon(class: &'static str, label: &'static str) -> Element {
    rsx! {
        svg {
            class,
            view_box: "0 0 194 168",
            fill: "none",
            stroke: "#71a554",
            stroke_linejoin: "round",
            role: "img",
            "aria-label": label,
            "data-card-icon": "sdf",
            path {
                stroke_width: "7.8",
                d: "M111.6 4.6H45.9c-8.1 0-14.7 6.6-14.7 14.7v130.3c0 8.1 6.6 14.7 14.7 14.7h102c8.1 0 14.7-6.6 14.7-14.7v-94L111.6 4.6Z"
            }
            path {
                stroke_width: "6.7",
                d: "M162.6 55.6h-36.3c-8.1 0-14.7-6.6-14.7-14.7V4.6"
            }
            text {
                x: "49.8",
                y: "113.9",
                fill: "#71a554",
                stroke: "none",
                font_size: "48",
                font_weight: "700",
                font_family: "Arial, sans-serif",
                "SDF"
            }
        }
    }
}
