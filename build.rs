use std::{env, fs, path::PathBuf};

use cosmol_viewer_core::{scene::Scene, shapes::Molecule};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let scene = build_home_scene();
    let bytes = postcard::to_allocvec(&scene).expect("serialize home scene");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR should be set"));
    fs::write(out_dir.join("home_scene.postcard"), bytes).expect("write precomputed home scene");
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
        .enable_outline(0.1);

    let mut scene = Scene::new();
    scene.set_scale(0.99);
    scene.recenter(mol.get_center());
    scene.add_shape(mol);
    scene.set_transparent_background(true);
    scene.set_zoom_disabled(true);
    scene.set_auto_rotate(true, 20.0);

    scene
}
