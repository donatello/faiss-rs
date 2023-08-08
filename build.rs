use std::env;
use std::path::Path;

fn main() {
    let faiss_lib = env::var("FAISS_LIB_DIR").unwrap();
    let lib_path = Path::new(&faiss_lib);
    assert!(lib_path.is_dir());
    assert!(lib_path.is_absolute());
    println!("cargo:rustc-link-search=/.faiss_c_lib");

    println!("cargo:rustc-link-lib=faiss_c");
    println!("cargo:rustc-link-lib=faiss");

    println!("cargo:rerun-if-changed=build.rs");
}
