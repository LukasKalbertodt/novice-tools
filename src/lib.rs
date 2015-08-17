//! This crate provides very simple functions that can be used by programming
//! beginners. Especially reading values from standard input (the user) is
//! very confusing for beginners.
//!
//! How to use this crate
//! =====================
//! First you need to put this into your `Cargo.toml`:
//! ```
//! [dependencies]
//! novice-tools = "*"
//! ```
//!
//! Next you need to put this in your crate-root (usually `main.rs`):
//! ```
//! extern crate novice_tools;
//! use novice_tools::*;
//! ```
//!

pub mod io;

pub use io::*;
