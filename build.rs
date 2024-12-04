fn main() {
    let sdftd3_dir =
        std::env::var("SDFTD3_DIR").unwrap_or("/home/a/miniconda3/envs/pyscf-pip/lib".to_string());
    println!("cargo:rustc-link-search=native={sdftd3_dir}");
    println!("cargo:rustc-link-lib=s-dftd3");
}
