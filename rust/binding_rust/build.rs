use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=wiringx");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_path = out_path.join("bindings.rs");
    bindings
        .write_to_file(&bindings_path)
        .expect("Couldn't write bindings!");

    println!("Generated bindings at: {:?}", bindings_path);

    let dest_path = PathBuf::from("binding_rust/bindings.rs");
    fs::copy(&bindings_path, dest_path).expect("Couldn't copy bindings!");
}
