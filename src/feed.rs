use std::default::Default;

use serde_json;

use errors::*;
use item::Item;
use builder::Builder;

const VERSION_1: &'static str = "https://jsonfeed.org/version/1";

/// Represents a single feed
///
/// # Examples
///
/// ```rust
/// // Serialize a feed object to a JSON string
///
/// # extern crate jsonfeed;
/// # use std::default::Default;
/// # use jsonfeed::Feed;
/// # fn main() {
/// let feed: Feed = Feed::default();
/// assert_eq!(
///     feed.to_string().unwrap(),
///     "{\"version\":\"https://jsonfeed.org/version/1\",\"title\":\"\",\"items\":[]}"
/// );
/// # }
/// ```
///
/// ```rust
/// // Deserialize a feed objects from a JSON String
///
/// # extern crate jsonfeed;
/// # use jsonfeed::Feed;
/// # fn main() {
/// let json = "{\"version\":\"https://jsonfeed.org/version/1\",\"title\":\"\",\"items\":[]}";
/// let feed: Feed = jsonfeed::from_str(&json).unwrap();
/// assert_eq!(
///     feed,
///     Feed::default()
/// );
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Feed {
    pub version: String,
    pub title: String,
    pub items: Vec<Item>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_page_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hubs: Option<Vec<Hub>>,
}

impl Feed {
    /// Serialize a Feed to a JSON Feed string
    pub fn to_string(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    /// Pretty-print a Feed to a JSON Feed string
    pub fn to_string_pretty(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn builder() -> Builder {
        Builder::new()
    }
}

impl Default for Feed {
    fn default() -> Feed {
        Feed {
            version: VERSION_1.to_string(),
            title: "".to_string(),
            items: vec![],
            home_page_url: None,
            feed_url: None,
            description: None,
            user_comment: None,
            next_url: None,
            icon: None,
            favicon: None,
            author: None,
            expired: None,
            hubs: None,
        }
    }
}

/// Represents the `content_html` and `content_text` attributes of an item
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Content {
    Html(String),
    Text(String),
    Both(String, String),
}

/// Represents an `attachment` for an item
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Attachment {
    url: String,
    mime_type: String,
    title: Option<String>,
    size_in_bytes: f64,
    duration_in_seconds: f64,
}

/// Represents an `author` in both a feed and a feed item
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Author {
    name: Option<String>,
    url: Option<String>,
    avatar: Option<String>,
}

/// Represents a `hub` for a feed
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Hub {
    #[serde(rename = "type")]
    type_: String,
    url: String,
}


