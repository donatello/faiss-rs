use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rustc-link-lib=faiss_c");
    println!("cargo:rustc-link-lib=faiss");

    let faiss_src = env::var("FAISS_SRC_DIR").unwrap();
    let library_path = Path::new(&faiss_src);
    assert!(library_path.is_dir());

    cxx_build::bridge("src/multibuf.rs")
        .file("src/cpp/multibuf.cpp")
        .include(library_path)
        .flag_if_supported("-std=c++17")
        .compile("multibuf");

    println!("cargo:rerun-if-changed=src/multibuf.rs");
    println!("cargo:rerun-if-changed=cpp/multibuf.cpp");
    println!("cargo:rerun-if-changed=cpp/multibuf.h");
}
