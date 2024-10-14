use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = PathBuf::from(out_dir).join("bindings.rs");
    let bindings = r#"
        pub type FOO_ULONG = u64;

        pub const PUBLIC_KEY: FOO_ULONG = 0x000000042;
        pub const PRIVATE_KEY: FOO_ULONG = 0x00000043;
    "#;
    fs::write(&dest_path, bindings).unwrap();
}
