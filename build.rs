use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    let src = ["vendor/cpplib/bar.cpp"];

    cc::Build::new()
        .cpp(true)
        .files(src.iter())
        .compile("mybar");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-xc++")
        .clang_arg("-Ivendor/cpplib")
        // .allowlist_function("barfunc")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
