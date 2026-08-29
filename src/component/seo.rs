use dioxus::prelude::*;

const SEO_KEYWORDS: &str =
    "Rust cheminformatics, COSMolKit, browser-based cheminformatics, WebAssembly molecular tools";
const RSS_FEED_URL: &str = "https://tools.cosmol.org/feed.xml";

#[component]
pub fn Seo(
    title: String,
    description: String,
    canonical: String,
    #[props(default)] published_at: Option<String>,
    #[props(default)] author_name: Option<String>,
    #[props(default)] author_email: Option<String>,
) -> Element {
    let json_ld = json_ld(
        &title,
        &description,
        &canonical,
        published_at.as_deref(),
        author_name.as_deref(),
        author_email.as_deref(),
    );

    #[cfg(target_arch = "wasm32")]
    {
        use_effect(move || sync_browser_seo(&title, &description, &canonical, &json_ld));
        rsx! {}
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        rsx! {
            document::Title { "{title}" }
            document::Meta { name: "description", content: "{description}" }
            document::Meta { name: "keywords", content: SEO_KEYWORDS }
            document::Link { rel: "canonical", href: "{canonical}" }
            document::Link {
                rel: "alternate",
                r#type: "application/rss+xml",
                title: "COSMolKit Blog",
                href: RSS_FEED_URL,
            }
            document::Meta { property: "og:title", content: "{title}" }
            document::Meta { property: "og:description", content: "{description}" }
            document::Meta { property: "og:url", content: "{canonical}" }
            document::Meta { property: "og:type", content: "website" }
            document::Script {
                r#type: "application/ld+json",
                "{json_ld}"
            }
        }
    }
}

fn json_escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len() + 2);
    escaped.push('"');
    for character in value.chars() {
        match character {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '<' => escaped.push_str("\\u003c"),
            '>' => escaped.push_str("\\u003e"),
            '&' => escaped.push_str("\\u0026"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            '\u{08}' => escaped.push_str("\\b"),
            '\u{0c}' => escaped.push_str("\\f"),
            character if character.is_control() => {
                use std::fmt::Write;
                write!(escaped, "\\u{:04x}", character as u32).expect("String write cannot fail");
            }
            character => escaped.push(character),
        }
    }
    escaped.push('"');
    escaped
}

fn json_ld(
    title: &str,
    description: &str,
    canonical: &str,
    published_at: Option<&str>,
    author_name: Option<&str>,
    author_email: Option<&str>,
) -> String {
    let title = json_escape(title);
    let description = json_escape(description);
    let website_id = json_escape("https://tools.cosmol.org/#website");
    let software_id = json_escape("https://kit.cosmol.org/#softwareapplication");
    let webpage_id = json_escape(&format!("{canonical}#webpage", canonical = canonical));
    let canonical = json_escape(canonical);
    let page = match (published_at, author_name, author_email) {
        (Some(published_at), Some(author_name), Some(author_email)) => format!(
            r#"{{"@type":"BlogPosting","@id":{webpage_id},"url":{canonical},"headline":{title},"description":{description},"datePublished":{published_at},"author":{{"@type":"Person","name":{author_name},"email":{author_email}}},"publisher":{{"@type":"Organization","name":"COSMol Studio","url":"https://github.com/cosmol-studio"}},"mainEntityOfPage":{{"@id":{webpage_id}}},"about":{{"@id":{software_id}}},"isPartOf":{{"@id":{website_id}}},"inLanguage":"en"}}"#,
            webpage_id = webpage_id,
            canonical = canonical,
            title = title,
            description = description,
            published_at = json_escape(published_at),
            author_name = json_escape(author_name),
            author_email = json_escape(&format!("mailto:{author_email}")),
            website_id = website_id,
            software_id = software_id,
        ),
        _ => format!(
            r#"{{"@type":"WebPage","@id":{webpage_id},"url":{canonical},"name":{title},"description":{description},"about":{{"@id":{software_id}}},"isPartOf":{{"@id":{website_id}}},"inLanguage":"en"}}"#,
            webpage_id = webpage_id,
            canonical = canonical,
            title = title,
            description = description,
            website_id = website_id,
            software_id = software_id,
        ),
    };

    format!(
        r#"{{"@context":"https://schema.org","@graph":[{{"@type":"WebSite","@id":{website_id},"url":"https://tools.cosmol.org/","name":"COSMolKit Tools Web","inLanguage":"en","publisher":{{"@type":"Organization","name":"COSMol Studio","url":"https://github.com/cosmol-studio"}}}},{{"@type":"SoftwareApplication","@id":{software_id},"name":"COSMolKit","applicationCategory":"DeveloperApplication","operatingSystem":["Linux","macOS","Windows"],"url":"https://kit.cosmol.org/","downloadUrl":"https://pypi.org/project/cosmolkit/","codeRepository":"https://github.com/cosmol-studio/COSMolKit","description":"Rust-native cheminformatics and structural biology toolkit","publisher":{{"@type":"Organization","name":"COSMol Studio","url":"https://github.com/cosmol-studio"}}}},{page}]}}"#,
        website_id = website_id,
        software_id = software_id,
        page = page,
    )
}

#[cfg(target_arch = "wasm32")]
fn sync_browser_seo(title: &str, description: &str, canonical: &str, json_ld: &str) {
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
    sync_unique_head_element(
        &document,
        "link[rel='alternate'][type='application/rss+xml']",
        "link",
        &[
            ("rel", "alternate"),
            ("type", "application/rss+xml"),
            ("title", "COSMolKit Blog"),
            ("href", RSS_FEED_URL),
        ],
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
    sync_unique_head_element(
        &document,
        "script[type='application/ld+json']",
        "script",
        &[("type", "application/ld+json")],
        Some(json_ld),
    );
}

#[cfg(target_arch = "wasm32")]
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
