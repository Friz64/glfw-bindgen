#![allow(rustdoc::broken_intra_doc_links)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![doc = include_str!("../README.md")]

#[link(name = "glfw3", kind = "static")]
extern "C" {}

#[cfg(target_family = "windows")]
#[link(name = "gdi32")]
#[link(name = "shell32")]
extern "C" {}

#[cfg(target_os = "macos")]
#[link(name = "Cocoa", kind = "framework")]
#[link(name = "IOKit", kind = "framework")]
#[link(name = "CoreFoundation", kind = "framework")]
#[link(name = "QuartzCore", kind = "framework")]
extern "C" {}

#[cfg(all(target_family = "unix", not(target_os = "macos")))]
#[link(name = "X11")]
extern "C" {}

#[cfg(all(target_family = "unix", not(target_os = "macos"), feature = "wayland"))]
#[link(name = "wayland-client")]
extern "C" {}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
