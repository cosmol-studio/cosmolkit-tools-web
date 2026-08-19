use std::{collections::HashSet, fs, path::Path};

const FINISHED_ROUTES: [(&str, &str); 7] = [
    ("/", "src/page/home.rs"),
    ("/tools", "src/page/tools.rs"),
    ("/tools/smiles-to-svg", "src/page/smiles_to_svg.rs"),
    ("/tools/format-converter", "src/page/format_converter.rs"),
    (
        "/tools/conformer-generator",
        "src/page/conformer_generator.rs",
    ),
    ("/tools/inchi", "src/page/inchi.rs"),
    ("/ecosystem", "src/page/ecosystem.rs"),
];

fn production_url(route: &str) -> String {
    if route == "/" {
        "https://tools.cosmol.org/".to_string()
    } else {
        format!("https://tools.cosmol.org{route}")
    }
}

#[test]
fn robots_allows_crawling_and_references_the_production_sitemap() {
    let robots = fs::read_to_string("robots.txt").expect("robots.txt should exist");
    assert!(robots.contains("User-agent: *"));
    assert!(robots.contains("Allow: /"));
    assert!(robots.contains("Sitemap: https://tools.cosmol.org/sitemap.xml"));
}

#[test]
fn sitemap_contains_each_finished_route_once_and_excludes_pains() {
    let sitemap = fs::read_to_string("sitemap.xml").expect("sitemap.xml should exist");
    let mut expected_urls = HashSet::new();

    for (route, _) in FINISHED_ROUTES {
        let url = production_url(route);
        assert!(expected_urls.insert(url.clone()));
        assert_eq!(
            sitemap.matches(&format!("<loc>{url}</loc>")).count(),
            1,
            "{url} must occur exactly once"
        );
    }

    assert!(!sitemap.contains("/tools/check-pains"));
    for url in sitemap
        .split("<loc>")
        .skip(1)
        .filter_map(|entry| entry.split_once("</loc>"))
    {
        assert!(url.0.starts_with("https://tools.cosmol.org"));
        assert!(expected_urls.contains(url.0));
    }
}

#[test]
fn every_public_route_declares_complete_metadata() {
    let mut pages = FINISHED_ROUTES.to_vec();
    pages.push(("/tools/check-pains", "src/page/check_pains.rs"));

    for (route, source_path) in pages {
        assert!(Path::new(source_path).exists(), "missing {source_path}");
        let source = fs::read_to_string(source_path).expect("page source should be readable");
        assert_eq!(
            source.matches("Seo {").count(),
            1,
            "missing SEO component in {source_path}"
        );
        assert!(source.contains("title:"), "missing title in {source_path}");
        assert!(
            source.contains("description:"),
            "missing description in {source_path}"
        );
        assert!(
            source.contains(&format!("canonical: \"{}\"", production_url(route))),
            "missing production canonical in {source_path}"
        );
    }
}

#[test]
fn seo_component_emits_standard_and_open_graph_tags() {
    let source = fs::read_to_string("src/component/seo.rs").expect("SEO component should exist");
    for marker in [
        "document::Title",
        "name: \"description\"",
        "rel: \"canonical\"",
        "property: \"og:title\"",
        "property: \"og:description\"",
        "property: \"og:url\"",
        "property: \"og:type\"",
    ] {
        assert!(source.contains(marker), "SEO component is missing {marker}");
    }
}

#[test]
fn initial_html_contains_a_self_removing_loading_state() {
    let template = fs::read_to_string("index.html").expect("initial HTML template should exist");
    let app = fs::read_to_string("src/main.rs").expect("app source should exist");
    let main_start = template
        .find("<div id=\"main\">")
        .expect("missing main mount");
    let loader = template
        .find("<div id=\"initial-template-loader\"")
        .expect("missing initial loader");

    assert!(
        loader > main_start,
        "loader must be inside the Dioxus mount"
    );
    assert!(template.contains("<title>{app_title}</title>"));
    assert!(template.contains("prefers-reduced-motion: reduce"));
    assert!(!template.contains("<script"));
    assert!(template.contains("body:has(#wasm-ready) #initial-template-loader"));
    assert!(!template.contains("initial-loader-molecule"));
    assert!(app.contains("id: \"wasm-ready\""));
    assert!(app.contains("InitialLoader {}"));
    assert!(!app.contains("initial-loader-molecule"));
    assert!(app.contains("use_effect(move || visible.set(false))"));
}
