//! JSONFeed Parser
//!

extern crate serde;
#[macro_use] extern crate serde_derive;
#[cfg(test)] extern crate serde_json;

mod item;

pub use item::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Feed {
    version: String,
    title: String,
    items: Vec<Item>,
    home_page_url: Option<String>,
    feed_url: Option<String>,
    description: Option<String>,
    user_comment: Option<String>,
    next_url: Option<String>,
    icon: Option<String>,
    favicon: Option<String>,
    author: Option<Author>,
    expired: Option<bool>,
    hubs: Option<Vec<Hub>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Content {
    Html(String),
    Text(String),
    Both(String, String),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Attachment {
    url: String,
    mime_type: String,
    title: Option<String>,
    size_in_bytes: f64,
    duration_in_seconds: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Author {
    name: Option<String>,
    url: Option<String>,
    avatar: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Hub {
    #[serde(rename = "type")]
    type_: String,
    url: String,
}

