// setup ./link as a linker search path
fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");

    println!("cargo:rustc-link-search={manifest_dir}/link");
}
