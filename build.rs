use std::{
    env, fs,
    path::{Path, PathBuf},
};

use cosmol_viewer_core::{scene::Scene, shapes::Molecule};
use pulldown_cmark::{Options, Parser, html};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=content/rust-cheminformatics.md");
    println!("cargo:rerun-if-changed=content/rust-cheminformatics-state-management.md");
    println!("cargo:rerun-if-changed=content/rust-cheminformatics-source-porting.md");
    println!("cargo:rerun-if-changed=content/rust-cheminformatics-validation.md");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR should be set"));
    build_markdown_article(
        "content/rust-cheminformatics.md",
        "rust_cheminformatics.html",
        &out_dir,
    );
    build_markdown_article(
        "content/rust-cheminformatics-state-management.md",
        "rust_cheminformatics_state_management.html",
        &out_dir,
    );
    build_markdown_article(
        "content/rust-cheminformatics-source-porting.md",
        "rust_cheminformatics_source_porting.html",
        &out_dir,
    );
    build_markdown_article(
        "content/rust-cheminformatics-validation.md",
        "rust_cheminformatics_validation.html",
        &out_dir,
    );

    let scene = build_home_scene();
    let bytes = postcard::to_allocvec(&scene).expect("serialize home scene");

    fs::write(out_dir.join("home_scene.postcard"), bytes).expect("write precomputed home scene");
}

fn build_markdown_article(source: &str, output: &str, out_dir: &Path) {
    let markdown = fs::read_to_string(source).expect("read Markdown article");
    let parser = Parser::new_ext(&markdown, Options::all());
    let mut rendered = String::with_capacity(markdown.len());
    html::push_html(&mut rendered, parser);
    fs::write(out_dir.join(output), rendered).expect("write rendered Markdown article");
}

fn build_home_scene() -> Scene {
    let base = cosmolkit::Molecule::from_smiles("O=[N+]([O-])c1ccc2ccccc2c1").unwrap();
    let mut params = cosmolkit::EmbedParameters::etkdg_v3();
    params.random_seed = 0xF00D;
    params.num_threads = 1;

    let cosmolkit_mol = base
        .with_hydrogens()
        .unwrap()
        .with_3d_conformer_with_params(params)
        .unwrap();

    let mol = Molecule::from_cosmolkit(&cosmolkit_mol)
        .unwrap()
        .centered()
        .set_outline(true, "#ffffff", 0.05);

    let mut scene = Scene::new();
    scene.set_scale(0.99);
    scene.recenter(mol.get_center());
    scene.add_shape(mol);
    scene.set_transparent_background(true);
    scene.set_zoom_disabled(true);
    scene.set_auto_rotate(true, 20.0);

    scene
}
