use bindgen::{Builder, CargoCallbacks};
use std::env::var;
use std::path::PathBuf;

////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    println!("cargo:rerun-if-changed=build.h");

    let bindings = Builder::default()
        .header("build.h")
        .clang_arg("-I/usr/src/cddl/compat/opensolaris/include")
        .clang_arg("-I/usr/src/sys/contrib/openzfs/lib/libspl/include/os/freebsd")
        .clang_arg("-I/usr/src/sys/contrib/openzfs/include")
        .clang_arg("-I/usr/src")
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(var("OUT_DIR").expect("OUT_DIR not set"));

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings!");
}
