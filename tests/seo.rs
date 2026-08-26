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

const CORE_CONVERSION_ROUTES: [(&str, &str); 17] = [
    (
        "/smiles-converter",
        "SMILES Converter Online — Powered by Rust | COSMolKit",
    ),
    (
        "/sdf-to-smiles",
        "SDF to SMILES Converter Online — Powered by Rust | COSMolKit",
    ),
    (
        "/mol-to-smiles",
        "MOL to SMILES Converter Online — Powered by Rust | COSMolKit",
    ),
    (
        "/mol2-to-smiles",
        "MOL2 to SMILES Converter Online — Powered by Rust | COSMolKit",
    ),
    (
        "/pdb-to-smiles",
        "PDB to SMILES Converter Online — Powered by Rust | COSMolKit",
    ),
    (
        "/mmcif-to-smiles",
        "mmCIF to SMILES Converter Online — Powered by Rust | COSMolKit",
    ),
    (
        "/xyz-to-smiles",
        "XYZ to SMILES Converter Online — Powered by Rust | COSMolKit",
    ),
    (
        "/smiles-to-sdf",
        "SMILES to SDF Converter Online — Powered by Rust | COSMolKit",
    ),
    (
        "/smiles-to-mol",
        "SMILES to MOL Converter Online — Powered by Rust | COSMolKit",
    ),
    (
        "/smiles-to-pdb",
        "SMILES to PDB Converter Online — Powered by Rust | COSMolKit",
    ),
    (
        "/mol2-to-sdf",
        "MOL2 to SDF Converter Online — Powered by Rust | COSMolKit",
    ),
    (
        "/mol2-to-pdb",
        "MOL2 to PDB Converter Online — Powered by Rust | COSMolKit",
    ),
    (
        "/sdf-to-mol",
        "SDF to MOL Converter Online — Powered by Rust | COSMolKit",
    ),
    (
        "/sdf-to-pdb",
        "SDF to PDB Converter Online — Powered by Rust | COSMolKit",
    ),
    (
        "/pdb-to-sdf",
        "PDB to SDF Converter Online — Powered by Rust | COSMolKit",
    ),
    (
        "/mmcif-to-pdb",
        "mmCIF to PDB Converter Online — Powered by Rust | COSMolKit",
    ),
    (
        "/xyz-to-sdf",
        "XYZ to SDF Converter Online — Powered by Rust | COSMolKit",
    ),
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
fn indexnow_key_and_deployment_notification_are_configured() {
    const KEY: &str = "b2d03e0f-cc00-4050-8e29-3316108be26b";
    let key_file = format!("{KEY}.txt");
    let key_contents = fs::read_to_string(&key_file).expect("IndexNow key file should exist");
    let workflow = fs::read_to_string(".github/workflows/depoly.yml")
        .expect("deployment workflow should exist");
    let notifier =
        fs::read_to_string("scripts/notify_indexnow.py").expect("IndexNow notifier should exist");

    assert_eq!(key_contents.trim(), KEY);
    assert!(workflow.contains(&format!("INDEXNOW_KEY: {KEY}")));
    assert!(workflow.contains("cp \"${INDEXNOW_KEY}.txt\""));
    assert!(workflow.contains("python scripts/notify_indexnow.py --dry-run"));
    assert!(workflow.contains("python scripts/notify_indexnow.py"));
    assert!(workflow.contains("github.event_name == 'push'"));
    assert!(notifier.contains("https://api.indexnow.org/indexnow"));
    assert!(notifier.contains("sitemap_urls(args.sitemap)"));
    assert!(!notifier.contains("/check-pains"));
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

    for route in [
        "/blog",
        "/rust-cheminformatics",
        "/rdkit-alternative-rust",
        "/rust-cheminformatics-libraries",
        "/validation",
    ] {
        let url = production_url(route);
        assert!(expected_urls.insert(url.clone()));
        assert_eq!(
            sitemap.matches(&format!("<loc>{url}</loc>")).count(),
            1,
            "{url} must occur exactly once"
        );
    }

    for (route, _) in CORE_CONVERSION_ROUTES {
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
    let mut sitemap_urls = HashSet::new();
    for url in sitemap
        .split("<loc>")
        .skip(1)
        .filter_map(|entry| entry.split_once("</loc>"))
    {
        assert!(url.0.starts_with("https://tools.cosmol.org"));
        assert!(
            sitemap_urls.insert(url.0.to_string()),
            "duplicate sitemap URL: {}",
            url.0
        );
    }
    assert!(expected_urls.is_subset(&sitemap_urls));
}

#[test]
fn conversion_routes_have_unique_titles_and_canonicals() {
    let source = fs::read_to_string("src/page/conversion_routes.rs")
        .expect("conversion route source should exist");
    assert!(!source.contains("Free & Secure Online Tool"));
    assert!(source.contains("https://tools.cosmol.org/{slug}"));
    assert!(source.contains("Converter Online — Powered by Rust | COSMolKit"));
    assert!(source.contains("fn has_dedicated_route"));
    assert!(source.contains("unsupported molecular conversion route"));
    assert!(source.contains("from_format_ids(\"pdb\", \"svg\").is_none()"));
    assert!(source.contains("https://tools.cosmol.org/smiles-to-svg"));

    for (route, title) in CORE_CONVERSION_ROUTES {
        assert!(title.contains("Converter Online"));
        assert!(title.contains("Powered by Rust"));
        assert!(title.ends_with("| COSMolKit"));
        assert!(!route.is_empty());
    }
}

#[test]
fn production_checks_reject_unsupported_conversion_pages() {
    let workflow =
        fs::read_to_string(".github/workflows/depoly.yml").expect("workflow should exist");
    let check = fs::read_to_string("scripts/check_ssg_output.py").expect("SSG check should exist");

    assert!(workflow.contains("rm -rf target/dx/cosmolkit-tools-web/release/web/public"));
    assert!(check.contains("unsupported conversion page was prerendered"));
    assert!(
        !fs::read_to_string("sitemap.xml")
            .unwrap()
            .contains("pdb-to-svg")
    );
}

#[test]
fn smiles_svg_preview_is_created_after_hydration() {
    let page =
        fs::read_to_string("src/page/smiles_to_svg.rs").expect("SMILES to SVG source should exist");
    let check = fs::read_to_string("scripts/check_ssg_output.py").expect("SSG check should exist");

    assert!(page.contains("None::<Result<RenderedMolecule, String>>"));
    assert!(page.contains("use_effect(move ||"));
    assert!(check.contains("prerendered SVG data URL is unsafe for hydration"));
}

#[test]
fn blog_routes_distinguish_published_and_placeholder_content() {
    let routes = fs::read_to_string("src/route.rs").expect("route source should exist");
    let navbar = fs::read_to_string("src/component/navbar.rs").expect("navbar should exist");
    let blog = fs::read_to_string("src/page/blog.rs").expect("blog pages should exist");
    let build = fs::read_to_string("build.rs").expect("build script should exist");

    for route in [
        "/blog",
        "/rust-cheminformatics",
        "/rdkit-alternative-rust",
        "/rust-cheminformatics-libraries",
        "/validation",
    ] {
        assert!(routes.contains(&format!("#[route(\"{route}\")]")));
        assert!(blog.contains(&format!("https://tools.cosmol.org{route}")));
    }

    assert!(navbar.contains("to: Route::Blog {}"));
    assert!(navbar.contains("\"Blog\""));
    assert_eq!(blog.matches("content: \"noindex, follow\"").count(), 0);
    assert!(blog.contains("PublishedBlogArticle { html: RUST_CHEMINFORMATICS_ARTICLE }"));
    assert!(
        blog.contains(
            "PublishedBlogArticle { html: RUST_CHEMINFORMATICS_STATE_MANAGEMENT_ARTICLE }"
        )
    );
    assert!(
        blog.contains("PublishedBlogArticle { html: RUST_CHEMINFORMATICS_SOURCE_PORTING_ARTICLE }")
    );
    assert!(
        blog.contains("PublishedBlogArticle { html: RUST_CHEMINFORMATICS_VALIDATION_ARTICLE }")
    );
    assert!(blog.contains("dangerous_inner_html: html"));
    assert!(build.contains("content/rust-cheminformatics.md"));
    assert!(build.contains("rust_cheminformatics.html"));
    assert!(build.contains("content/rust-cheminformatics-state-management.md"));
    assert!(build.contains("rust_cheminformatics_state_management.html"));
    assert!(build.contains("content/rust-cheminformatics-source-porting.md"));
    assert!(build.contains("rust_cheminformatics_source_porting.html"));
    assert!(build.contains("content/rust-cheminformatics-validation.md"));
    assert!(build.contains("rust_cheminformatics_validation.html"));
    assert!(!blog.contains("use cosmolkit"));
    assert!(!blog.contains("cosmol_viewer"));
}

#[test]
fn blog_index_matches_the_planned_non_batch_series() {
    let blog = fs::read_to_string("src/page/blog.rs").expect("blog source should exist");

    assert_eq!(blog.matches("published: true").count(), 4);
    assert_eq!(blog.matches("published: false").count(), 0);

    for title in [
        "Rust Cheminformatics Beyond RDKit Bindings",
        "Rust Cheminformatics State Management",
        "Rust Cheminformatics Porting",
        "Rust Cheminformatics Validation",
    ] {
        assert!(
            blog.contains(title),
            "blog is missing planned title {title}"
        );
    }

    assert!(!blog.contains("Rust Cheminformatics at Scale"));
    assert!(!blog.contains("Batch Molecular Processing"));
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
            source.contains(&production_url(route)),
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
        "name: \"keywords\"",
        "rel: \"canonical\"",
        "property: \"og:title\"",
        "property: \"og:description\"",
        "property: \"og:url\"",
        "property: \"og:type\"",
    ] {
        assert!(source.contains(marker), "SEO component is missing {marker}");
    }

    for keyword in ["Rust", "cheminformatics", "COSMolKit"] {
        assert!(
            source.contains(keyword),
            "SEO component is missing global keyword {keyword}"
        );
    }

    assert!(source.contains("sync_browser_seo"));
    assert!(source.contains("sync_unique_head_element"));
    assert!(source.contains("element.remove()"));
    assert!(source.contains("target_arch = \"wasm32\", feature = \"ssg\""));
}

#[test]
fn production_wasm_does_not_reinsert_ssg_document_assets() {
    let source = fs::read_to_string("src/main.rs").expect("app source should exist");

    assert!(source.contains("DocumentAssets {}"));
    assert!(source.contains("fn DocumentAssets() -> Element"));
    assert!(source.contains("target_arch = \"wasm32\", feature = \"ssg\""));
}

#[test]
fn shared_footer_reinforces_rust_cheminformatics_context() {
    let source = fs::read_to_string("src/component/navbar.rs").expect("navbar should exist");
    assert!(source.contains("Rust cheminformatics powered by COSMolKit"));
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
    let home = fs::read_to_string("src/page/home.rs").expect("home source should be readable");
    let tools = fs::read_to_string("src/page/tools.rs").expect("tools source should be readable");

    for icon in [
        "DepictionCardIcon",
        "FormatCardIcon",
        "ConformerCardIcon",
        "IdentifierCardIcon",
    ] {
        assert!(home.contains(icon));
        assert!(tools.contains(icon));
    }
    for icon in [
        "PropertiesCardIcon",
        "CanonicalCardIcon",
        "FilterAlertCardIcon",
    ] {
        assert!(tools.contains(icon));
    }
    for source in [&home, &tools] {
        assert!(!source.contains("src: MOLECULE_SVG"));
        assert!(!source.contains("src: SDF_SVG"));
    }

    for icon in [
        "depiction",
        "format",
        "conformer",
        "identifier",
        "properties",
        "canonical",
        "filter-alert",
    ] {
        assert!(icons.contains(&format!("\"data-card-icon\": \"{icon}\"")));
    }
    assert!(icons.matches("stroke: \"currentColor\"").count() >= 7);
}
