#[cfg(not(target_os = "macos"))]
fn main() {
    panic!("Cannot build for non OSX platforms")
}

#[cfg(target_os = "macos")]
fn main() {
    use std::env;
    use std::path::PathBuf;

    let bindings = bindgen::Builder::default()
        .header("notify.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings for endpoint security");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("notify_sys.rs"))
        .expect("Couldn't write bindings!");
}
