use std::{collections::HashSet, fs, path::Path};

const FINISHED_ROUTES: [(&str, &str); 9] = [
    ("/", "src/page/home.rs"),
    ("/tools", "src/page/tools.rs"),
    ("/smiles-to-svg", "src/page/smiles_to_svg.rs"),
    ("/format-converter", "src/page/format_converter.rs"),
    ("/conformer-generator", "src/page/conformer_generator.rs"),
    ("/inchi", "src/page/inchi.rs"),
    ("/molecular-properties", "src/page/molecular_properties.rs"),
    ("/smiles-canonicalizer", "src/page/smiles_canonicalizer.rs"),
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
fn legacy_tool_routes_redirect_to_root_level_urls() {
    let redirects = fs::read_to_string("_redirects").expect("Cloudflare redirects should exist");

    for (old_route, new_route) in [
        ("/tools/smiles-to-svg", "/smiles-to-svg"),
        ("/tools/format-converter", "/format-converter"),
        ("/tools/conformer-generator", "/conformer-generator"),
        ("/tools/inchi", "/inchi"),
        ("/tools/molecular-properties", "/molecular-properties"),
        ("/tools/smiles-canonicalizer", "/smiles-canonicalizer"),
        ("/tools/check-pains", "/check-pains"),
    ] {
        let rule = format!("{old_route} {new_route} 301");
        assert_eq!(
            redirects.lines().filter(|line| *line == rule).count(),
            1,
            "missing unique permanent redirect: {rule}"
        );
    }
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

    assert!(!sitemap.contains("/check-pains"));
    assert!(!sitemap.contains("https://tools.cosmol.org/tools/"));
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
    pages.push(("/check-pains", "src/page/check_pains.rs"));

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
fn public_tools_expose_supported_search_terms() {
    let source = fs::read_to_string("src/page/format_converter.rs")
        .expect("format converter source should exist");

    for conversion in [
        "SDF to SMILES",
        "SMILES to SDF",
        "MOL2 to SDF",
        "PDB to SDF",
        "mmCIF to SMILES",
        "XYZ to SMILES",
        "SMILES to SVG",
    ] {
        assert!(
            source.contains(conversion),
            "format converter is missing the supported search term {conversion}"
        );
    }

    for (source_path, search_terms) in [
        (
            "src/page/smiles_to_svg.rs",
            &[
                "SMILES renderer",
                "chemical structure drawing",
                "SMILES visualizer",
            ][..],
        ),
        (
            "src/page/conformer_generator.rs",
            &["SMILES to 3D", "ETKDG v3", "SMILES to PDB"][..],
        ),
        (
            "src/page/inchi.rs",
            &[
                "SMILES to InChI",
                "InChI to canonical SMILES",
                "InChIKey generator",
            ][..],
        ),
        (
            "src/page/molecular_properties.rs",
            &[
                "molecular formula",
                "molecular weight",
                "exact mass",
                "Crippen logP",
                "rotatable bonds",
            ][..],
        ),
        (
            "src/page/smiles_canonicalizer.rs",
            &[
                "Canonical SMILES",
                "Isomeric SMILES",
                "Kekulized SMILES",
                "formal charge",
            ][..],
        ),
    ] {
        let source = fs::read_to_string(source_path).expect("tool page source should exist");
        for search_term in search_terms {
            assert!(
                source.contains(search_term),
                "{source_path} is missing the supported search term {search_term}"
            );
        }
    }
}

#[test]
fn primary_pages_expose_rust_and_cheminformatics_as_indexable_text() {
    for (source_path, browser_phrase) in [
        ("src/page/home.rs", "browser-native"),
        ("src/page/tools.rs", "browser-based"),
        ("src/page/ecosystem.rs", "browser-native"),
    ] {
        let source = fs::read_to_string(source_path).expect("page source should be readable");
        let lowercase_source = source.to_lowercase();
        for phrase in ["rust", "cheminformatics", browser_phrase] {
            assert!(
                lowercase_source.matches(phrase).count() >= 3,
                "{source_path} should include {phrase} in metadata and visible text"
            );
        }
    }

    let config = fs::read_to_string("Dioxus.toml").expect("Dioxus config should exist");
    let lowercase_config = config.to_lowercase();
    assert!(lowercase_config.contains("rust"));
    assert!(lowercase_config.contains("cheminformatics"));
    assert!(lowercase_config.contains("browser-native"));
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

#[test]
fn production_build_enables_route_level_wasm_splitting() {
    let manifest = fs::read_to_string("Cargo.toml").expect("Cargo manifest should exist");
    let workflow = fs::read_to_string(".github/workflows/depoly.yml")
        .expect("deployment workflow should exist");
    let navbar = fs::read_to_string("src/component/navbar.rs").expect("navbar source should exist");
    let app = fs::read_to_string("src/main.rs").expect("app source should exist");

    assert!(manifest.contains("dioxus/wasm-split"));
    assert!(manifest.contains("dioxus-router/wasm-split"));
    assert!(manifest.contains("ssg = []"));
    assert!(workflow.contains("--features \"ssg wasm-split\""));
    assert!(workflow.contains("--wasm-split"));
    assert!(workflow.contains("--fullstack true"));
    assert!(navbar.contains("SuspenseBoundary"));
    assert!(navbar.contains("Loading tool"));
    assert!(app.contains("root.set_inner_html(\"\")"));
}

#[test]
fn split_routes_use_embedded_card_icons() {
    let icons = fs::read_to_string("src/component/icon.rs").expect("icon source should exist");
    for source_path in ["src/page/home.rs", "src/page/tools.rs"] {
        let source = fs::read_to_string(source_path).expect("page source should be readable");
        assert!(source.contains("MoleculeCardIcon"));
        assert!(source.contains("SdfCardIcon"));
        assert!(!source.contains("src: MOLECULE_SVG"));
        assert!(!source.contains("src: SDF_SVG"));
    }

    assert!(icons.contains("\"data-card-icon\": \"molecule\""));
    assert!(icons.contains("\"data-card-icon\": \"sdf\""));
}
