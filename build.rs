fn main() {
    let dftd3_dir =
        std::env::var("DFTD3_DIR").unwrap_or("/home/a/miniconda3/envs/pyscf-pip/lib".to_string());
    let dftd3_static = std::env::var("DFTD3_STATIC").unwrap_or("".to_string());
    println!("cargo:rustc-link-search=native={dftd3_dir}");
    if dftd3_static.is_empty() {
        println!("cargo:rustc-link-lib=s-dftd3");
    } else {
        println!("cargo:rustc-link-lib=static=s-dftd3");
    }
}
