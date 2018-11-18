# XDG Base Directory Loader
This repository is a library for working with XDG base directories for *nix programs. 
This defines a consistent format for find and placing files on a system so a computer program
can find files on the system.

## Usage
Place the `xdg` library in your source tree's `Cargo.toml` file.
```toml
[dependencies]
xdg = "0.2"
```
The `xdg` library is also intended to be used as part of the build and installation process. To 
use it in the build process, place it in your source tree's `Cargo.toml` file as follows.
```toml
[build-dependencies]
xdg = "0.2"
```

## Dependencies
This library requires `Rust 2018 Edition (>= 1.30)` to compile. It depends on no other libraries.

## Compatibility
This library is meant to support any piece of software design to comply with the 
*XDG Base Directory Specification* The primary use case for this is Linux software.

## References
* [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html)
