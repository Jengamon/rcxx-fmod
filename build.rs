fn main() {
    cxx_build::bridge("src/lib.rs")  // returns a cc::Build
        .file("cpp/lib.cpp")
        .includes(vec!["fmod/inc", "."])
        .flag_if_supported("-std=c++11")
        .compile("rcxxfmod");

    println!("cargo:rerun-if-changed=src/**/**.rs");
    println!("cargo:rerun-if-changed=cpp/**/**.cpp");
    println!("cargo:rerun-if-changed=cpp/**/**.hpp");
    println!("cargo:rustc-link-search=fmod/lib");
    println!("cargo:rustc-link-lib=fmod");
    println!("cargo:rustc-link-lib=fmodstudio");
}