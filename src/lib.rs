//! Use chumsky in your proc-macros with chumsky-proc!
//!
//! Simply write your parsers which take streams of `RustToken`, then call
//! `stream_from_tokens` with a `TokenStream` to generate a stream that can be passed
//! to your parsers. Easy as pie!

#![warn(
    missing_docs,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    missing_abi,
    noop_method_call,
    pointer_structural_match,
    semicolon_in_expressions_from_macros,
    unused_import_braces,
    unused_lifetimes,
    clippy::missing_panics_doc,
    clippy::doc_markdown,
    clippy::ptr_as_ptr,
    clippy::cloned_instead_of_copied,
    clippy::unreadable_literal
)]

pub mod primitive;
mod regular;
mod span;
mod token;
pub(crate) mod utils;
// TODO: zero-copy, once it's released

pub use regular::*;
pub use span::RustSpan;
pub use token::RustToken;
