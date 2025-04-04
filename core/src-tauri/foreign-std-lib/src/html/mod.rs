//! *Html
//!
//! An Html type is an representation of an HTML-Element. This mapping is close to total.

#[cfg(feature = "c")]
/// C-Html
pub mod c_html;
#[cfg(feature = "rs")]
/// Rust-Html
pub mod rs_html;
#[cfg(feature = "ts")]
/// TypeScript Html
pub mod ts_html;
