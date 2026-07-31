fn main() {
    println!("cargo:rerun-if-env-changed=CARGO_MANIFEST_DIR");
    println!("cargo:rerun-if-changed=recipes");
    println!("cargo:rerun-if-changed=content");
}
