//! # jsapi: A full-featured Rust to JavaScript (asm.js/wasm) API wrapper.
//!
//! ## Interaction between the type systems
//!
//! Interacting with the poorly typed JavaScript API is hard. We get around it by using a construct
//! we call _the object pool_. Essentially, "pointers" into JavaScript objects (strings, maps,
//! arrays, etc.) are represented as indexes into a homogenous array.
//!
//! Obviously, this is an incredibly hacky solution, but it's deeply rooted in the flawed design of
//! JavaScript.

pub mod dom;
pub mod ffi;
pub mod local_storage;
pub mod location;
pub mod prelude;
pub mod string;
