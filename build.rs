fn main() {
    // search `DFTD3_DIR` for libs-dftd3
    if let Ok(dftd3_dir) = std::env::var("DFTD3_DIR") {
        println!("cargo:rustc-link-search=native={dftd3_dir}");
    };
    // search `REST_EXT_DIR` for libs-dftd3
    if let Ok(dftd3_dir) = std::env::var("REST_EXT_DIR") {
        println!("cargo:rustc-link-search=native={dftd3_dir}");
    };
    // static linking or anyway
    if let Ok(_) = std::env::var("DFTD3_STATIC") {
        println!("cargo:rustc-link-lib=static=s-dftd3");
    } else {
        println!("cargo:rustc-link-lib=s-dftd3");
    };
}
