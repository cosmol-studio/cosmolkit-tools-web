use dioxus::prelude::*;

const SEO_KEYWORDS: &str =
    "Rust cheminformatics, COSMolKit, browser-based cheminformatics, WebAssembly molecular tools";

#[component]
pub fn Seo(title: String, description: String, canonical: String) -> Element {
    #[cfg(all(target_arch = "wasm32", feature = "ssg"))]
    {
        use_effect(move || sync_browser_seo(&title, &description, &canonical));
        rsx! {}
    }

    #[cfg(not(all(target_arch = "wasm32", feature = "ssg")))]
    {
        rsx! {
            document::Title { "{title}" }
            document::Meta { name: "description", content: "{description}" }
            document::Meta { name: "keywords", content: SEO_KEYWORDS }
            document::Link { rel: "canonical", href: "{canonical}" }
            document::Meta { property: "og:title", content: "{title}" }
            document::Meta { property: "og:description", content: "{description}" }
            document::Meta { property: "og:url", content: "{canonical}" }
            document::Meta { property: "og:type", content: "website" }
        }
    }
}

#[cfg(all(target_arch = "wasm32", feature = "ssg"))]
fn sync_browser_seo(title: &str, description: &str, canonical: &str) {
    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        return;
    };

    sync_unique_head_element(&document, "title", "title", &[], Some(title));
    sync_unique_head_element(
        &document,
        "meta[name='description']",
        "meta",
        &[("name", "description"), ("content", description)],
        None,
    );
    sync_unique_head_element(
        &document,
        "meta[name='keywords']",
        "meta",
        &[("name", "keywords"), ("content", SEO_KEYWORDS)],
        None,
    );
    sync_unique_head_element(
        &document,
        "link[rel='canonical']",
        "link",
        &[("rel", "canonical"), ("href", canonical)],
        None,
    );
    for (property, content) in [
        ("og:title", title),
        ("og:description", description),
        ("og:url", canonical),
        ("og:type", "website"),
    ] {
        sync_unique_head_element(
            &document,
            &format!("meta[property='{property}']"),
            "meta",
            &[("property", property), ("content", content)],
            None,
        );
    }
}

#[cfg(all(target_arch = "wasm32", feature = "ssg"))]
fn sync_unique_head_element(
    document: &web_sys::Document,
    selector: &str,
    tag: &str,
    attributes: &[(&str, &str)],
    text: Option<&str>,
) {
    use wasm_bindgen::JsCast;

    let Ok(elements) = document.query_selector_all(selector) else {
        return;
    };

    let element = elements
        .item(0)
        .and_then(|node| node.dyn_into::<web_sys::Element>().ok())
        .or_else(|| {
            let element = document.create_element(tag).ok()?;
            let head = document.query_selector("head").ok().flatten()?;
            head.append_child(&element).ok()?;
            Some(element)
        });

    let Some(element) = element else {
        return;
    };
    for (name, value) in attributes {
        let _ = element.set_attribute(name, value);
    }
    if let Some(text) = text {
        element.set_text_content(Some(text));
    }

    for index in (1..elements.length()).rev() {
        if let Some(node) = elements.item(index) {
            if let Ok(element) = node.dyn_into::<web_sys::Element>() {
                element.remove();
            }
        }
    }
}
