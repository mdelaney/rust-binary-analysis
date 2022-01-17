extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    //println!("cargo:rustc-link-lib=capstone");
    println!("cargo:rustc-link-lib=capstone");
    println!("cargo:rustc-link-search=/opt/homebrew/lib");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    #[cfg(target_os = "linux")]
    let capstone_header = "/usr/include/capstone/capstone.h";

    // this assumes the default location when installing capstone with brew, probably
    // this can/should be done differently
    #[cfg(target_os = "macos")]
    let capstone_header = "/opt/homebrew/include/capstone/capstone.h";

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(capstone_header)
        .disable_name_namespacing()
        .impl_debug(true)
        //        .rustified_enum("cs_err|cs_group_type|cs_opt_value")
        .rustified_enum(".*")
        .whitelist_function("cs_.*")
        .whitelist_type(".*")
        //        .whitelist_type("cs_.*")
        .whitelist_type(".*_arm(64)?_.*")
        .whitelist_type(".*_x86_.*")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        //.parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let _out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        //        .write_to_file(out_path.join("bindings.rs"))
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}
