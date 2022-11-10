use std::{env, path::PathBuf};

#[cfg(feature = "wayland")]
const GLFW_USE_WAYLAND: &str = "ON";
#[cfg(not(feature = "wayland"))]
const GLFW_USE_WAYLAND: &str = "OFF";

fn main() {
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .allowlist_function("glfw.*")
        .allowlist_var("GLFW.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let libdir = "lib";
    let cmake = cmake::Config::new("./glfw")
        .define("CMAKE_INSTALL_LIBDIR", libdir)
        .define("GLFW_BUILD_DOCS", "OFF")
        .define("GLFW_BUILD_EXAMPLES", "OFF")
        .define("GLFW_BUILD_TESTS", "OFF")
        .define("GLFW_USE_WAYLAND", GLFW_USE_WAYLAND)
        .build();
    println!(
        "cargo:rustc-link-search=native={}",
        cmake.join(libdir).display()
    );
}
