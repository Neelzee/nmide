//! *Attr
//!
//! An Attr type is a representation of an HTML-attribute. This mapping is not total, and will not
//! be total, but it does include some of the bare-essentials to create a simple Plugin.

/// Rust-Attr
#[cfg(feature = "rs")]
pub mod rs_attrs;
#[cfg(feature = "ts")]
/// TypeScript Attr
pub mod ts_attrs;
