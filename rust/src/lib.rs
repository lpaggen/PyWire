#![forbid(unsafe_code)]

//! Owned Rust data structures for the Python 3.14 abstract syntax tree.
//!
//! [`PBDecoder`] reads PyWire Protocol Buffer files and recursively converts
//! every generated wire message into the independent types in [`ast`].

pub mod ast;
pub mod pb_decoder;

/// Prost-generated messages for PyWire's Python AST transport schema.
///
/// Most users should decode through [`PBDecoder`]. These types are public so
/// callers that already hold an in-memory protobuf message can use the
/// decoder's `convert_*` methods without serializing it first.
pub mod pb {
    include!(concat!(env!("OUT_DIR"), "/python.ast.rs"));
}

pub use pb_decoder::PBDecoder;
