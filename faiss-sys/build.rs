use std::env;
use std::path::Path;

fn main() {
    let faiss_src = env::var("FAISS_SRC_DIR").unwrap();
    let src_path = Path::new(&faiss_src);
    assert!(src_path.is_dir());
    assert!(src_path.is_absolute());

    let faiss_lib = env::var("FAISS_LIB_DIR").unwrap();
    let lib_path = Path::new(&faiss_lib);
    assert!(lib_path.is_dir());
    assert!(lib_path.is_absolute());
    println!("cargo:rustc-link-search={}", lib_path.to_str().unwrap());

    println!("cargo:rustc-link-lib=faiss_c");
    println!("cargo:rustc-link-lib=faiss");

    cxx_build::bridge("src/multibuf.rs")
        .file("src/cpp/multibuf.cpp")
        .include(src_path)
        .flag_if_supported("-std=c++17")
        .compile("multibuf");

    println!("cargo:rerun-if-changed=src/multibuf.rs");
    println!("cargo:rerun-if-changed=cpp/multibuf.cpp");
    println!("cargo:rerun-if-changed=cpp/multibuf.h");

    println!("cargo:rerun-if-changed=build.rs");
}
