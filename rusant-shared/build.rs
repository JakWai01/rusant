extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("project_dir: {}", project_dir); 
    println!("cargo:rustc-link-search={}/{}", project_dir, "cproject"); // the "-L" flag
    println!("cargo:rustc-link-lib=shared"); // the "-l" flag

    println!("cargo:rerun-if-changed=shared.h");

    let bindings = bindgen::Builder::default()
        .header("shared.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}