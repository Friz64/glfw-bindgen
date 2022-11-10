# glfw-bindgen

[![crates.io](https://img.shields.io/crates/v/glfw-bindgen)](https://crates.io/crates/glfw-bindgen)
[![docs.rs](https://docs.rs/glfw-bindgen/badge.svg)](https://docs.rs/glfw-bindgen)

Bindings to GLFW generated with
[rust-bindgen](https://rust-lang.github.io/rust-bindgen/).

While compiling this crate, it generates all the binding code and compiles GLFW
from source.

## Used GLFW version

Git revision
[`dd8a678`](https://github.com/glfw/glfw/commit/dd8a678a66f1967372e5a5e3deac41ebf65ee127).

## Features

`wayland`: Enables Wayland support. Please note that currently GLFW still
prefers X11 if available, though you can bypass this by setting `DISPLAY=`.

## Licensing

This project (and GLFW itself) is licensed under the [zlib License](LICENSE).
