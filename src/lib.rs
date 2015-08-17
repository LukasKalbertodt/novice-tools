//! This crate provides very simple functions that can be used by programming
//! beginners. Especially reading values from standard input (the user) is
//! very confusing for beginners.
//!
//! Currently all useful methods are in the `io` module.
//!
//! How to use this crate
//! =====================
//! First you need to put this into your `Cargo.toml`:
//!
//! ```ignore
//! [dependencies]
//! novice-tools = "*"
//! ```
//!
//! Next you need to put this in your crate-root (usually `main.rs`):
//!
//! ```ignore
//! extern crate novice_tools;
//! use novice_tools::*;
//! ```
//!

pub mod io;

pub use io::*;
