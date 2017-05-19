//! JSON Feed is a syndication format similar to ATOM and RSS, using JSON
//! instead of XML
//!
//! This crate can serialize and deserialize between JSON Feed strings
//! and Rust data structures. It also allows for programmatically building 
//! a JSON Feed
//!
//! Example:
//!
//! ```rust
//! extern crate jsonfeed;
//!
//! use jsonfeed::{Feed, Item};
//!
//! fn run() -> Result<(), jsonfeed::Error> {
//!     let j = r#"{
//!         "title": "my feed",
//!         "version": "https://jsonfeed.org/version/1",
//!         "items": []
//!     }"#;
//!     let feed: Feed = jsonfeed::from_str(j).unwrap();
//!
//!     let new_feed = Feed::builder()
//!                         .title("some other feed")
//!                         .item(Item::builder()
//!                                 .title("some item title")
//!                                 .content_html("<p>Hello, World</p>")
//!                                 .build()?)
//!                         .item(Item::builder()
//!                                 .title("some other item title")
//!                                 .content_text("Hello, World!")
//!                                 .build()?)
//!                         .build();
//!     println!("{}", new_feed.to_string().unwrap());
//!     Ok(())
//! }
//! fn main() {
//!     let _ = run();
//! }
//! ```

extern crate serde;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

mod errors;
mod item;
mod feed;
mod builder;

pub use errors::*;
pub use item::*;
pub use feed::{Feed, Author, Content, Attachment};

/// Attempts to convert a string slice to a Feed object
///
/// Example
///
/// ```rust
/// # extern crate jsonfeed;
/// # use jsonfeed::Feed;
/// # use std::default::Default;
/// # fn main() {
/// let json = r#"{"version": "https://jsonfeed.org/version/1", "title": "", "items": []}"#;
/// let feed: Feed = jsonfeed::from_str(&json).unwrap();
///
/// assert_eq!(feed, Feed::default());
/// # }
/// ```
pub fn from_str(s: &str) -> Result<Feed> {
    Ok(serde_json::from_str(s)?)
}

/// Deserialize a Feed object from an IO stream of JSON
pub fn from_reader<R: ::std::io::Read>(r: R) -> Result<Feed> {
    Ok(serde_json::from_reader(r)?)
}

/// Deserialize a Feed object from bytes of JSON text
pub fn from_slice<'a>(v: &'a [u8]) -> Result<Feed> {
    Ok(serde_json::from_slice(v)?)
}

/// Convert a serde_json::Value type to a Feed object
pub fn from_value(value: serde_json::Value) -> Result<Feed> {
    Ok(serde_json::from_value(value)?)
}
