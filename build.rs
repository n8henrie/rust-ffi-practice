extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=vendor/proj.h");

    println!("cargo:rustc-link-search=./vendor");
    println!("cargo:rustc-link-lib=proj");

    let bindings = bindgen::Builder::default()
        .header("vendor/proj.h")
        .clang_args(["-xc++", "-std=c++17"])
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .opaque_type("std::.*")
        .allowlist_type("Configuration")
        .allowlist_var("config")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
