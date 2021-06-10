#![allow(deprecated)]
use feed::{Author, Attachment};
use serde_with::skip_serializing_none;

/// Represents an item in a feed
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub language: Option<String>,
    pub url: Option<String>,
    pub external_url: Option<String>,
    pub title: Option<String>,
    pub content_html: Option<String>,
    pub content_text: Option<String>,
    pub summary: Option<String>,
    pub image: Option<String>,
    pub banner_image: Option<String>,
    pub date_published: Option<String>, // todo DateTime objects?
    pub date_modified: Option<String>,
    #[deprecated(since="0.3.0", note="Please use `authors` instead")]
    pub author: Option<Author>,
    pub authors: Option<Vec<Author>>,
    pub tags: Option<Vec<String>>,
    pub attachments: Option<Vec<Attachment>>,
}

impl Default for Item {
    fn default() -> Item {
        Item {
            id: "".to_string(),
            language: None,
            url: None,
            external_url: None,
            title: None,
            content_html: None,
            content_text: None,
            summary: None,
            image: None,
            banner_image: None,
            date_published: None,
            date_modified: None,
            author: None,
            authors: None,
            tags: None,
            attachments: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use feed::Author;
    use serde_json;

    #[test]
    #[allow(non_snake_case)]
    fn serialize_item__content_html() {
        let item = Item {
            id: "1".into(),
            url: Some("http://example.com/feed.json".into()),
            external_url: Some("http://example.com/feed.json".into()),
            title: Some("feed title".into()),
            content_html: Some("<p>content</p>".into()),
            summary: Some("feed summary".into()),
            image: Some("http://img.com/blah".into()),
            banner_image: Some("http://img.com/blah".into()),
            date_published: Some("2017-01-01 10:00:00".into()),
            date_modified: Some("2017-01-01 10:00:00".into()),
            authors: Some(
                vec![
                    Author::new()
                        .name("bob jones")
                        .url("http://example.com")
                        .avatar("http://img.com/blah")]),

            tags: Some(vec!["json".into(), "feed".into()]),
            attachments: Some(vec![]),
            ..Default::default()
        };
        assert_eq!(
            serde_json::to_string(&item).unwrap(),
            r#"{"id":"1","url":"http://example.com/feed.json","external_url":"http://example.com/feed.json","title":"feed title","content_html":"<p>content</p>","summary":"feed summary","image":"http://img.com/blah","banner_image":"http://img.com/blah","date_published":"2017-01-01 10:00:00","date_modified":"2017-01-01 10:00:00","authors":[{"name":"bob jones","url":"http://example.com","avatar":"http://img.com/blah"}],"tags":["json","feed"],"attachments":[]}"#
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn serialize_item__content_text() {
        let item = Item {
            id: "1".into(),
            url: Some("http://example.com/feed.json".into()),
            external_url: Some("http://example.com/feed.json".into()),
            title: Some("feed title".into()),
            content_text: Some("content".into()),
            summary: Some("feed summary".into()),
            image: Some("http://img.com/blah".into()),
            banner_image: Some("http://img.com/blah".into()),
            date_published: Some("2017-01-01 10:00:00".into()),
            date_modified: Some("2017-01-01 10:00:00".into()),
            authors: Some(vec![Author::new().name("bob jones").url("http://example.com").avatar("http://img.com/blah")]),
            tags: Some(vec!["json".into(), "feed".into()]),
            attachments: Some(vec![]),
            ..Default::default()
        };
        assert_eq!(
            serde_json::to_string(&item).unwrap(),
            r#"{"id":"1","url":"http://example.com/feed.json","external_url":"http://example.com/feed.json","title":"feed title","content_text":"content","summary":"feed summary","image":"http://img.com/blah","banner_image":"http://img.com/blah","date_published":"2017-01-01 10:00:00","date_modified":"2017-01-01 10:00:00","authors":[{"name":"bob jones","url":"http://example.com","avatar":"http://img.com/blah"}],"tags":["json","feed"],"attachments":[]}"#
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn serialize_item__content_both() {
        let item = Item {
            id: "1".into(),
            url: Some("http://example.com/feed.json".into()),
            external_url: Some("http://example.com/feed.json".into()),
            title: Some("feed title".into()),
            content_html: Some("<p>content</p>".into()),
            content_text: Some("content".into()),
            summary: Some("feed summary".into()),
            image: Some("http://img.com/blah".into()),
            banner_image: Some("http://img.com/blah".into()),
            date_published: Some("2017-01-01 10:00:00".into()),
            date_modified: Some("2017-01-01 10:00:00".into()),
            authors: Some(vec![Author::new().name("bob jones").url("http://example.com").avatar("http://img.com/blah")]),
            tags: Some(vec!["json".into(), "feed".into()]),
            attachments: Some(vec![]),
            ..Default::default()
        };
        assert_eq!(
            serde_json::to_string(&item).unwrap(),
            r#"{"id":"1","url":"http://example.com/feed.json","external_url":"http://example.com/feed.json","title":"feed title","content_html":"<p>content</p>","content_text":"content","summary":"feed summary","image":"http://img.com/blah","banner_image":"http://img.com/blah","date_published":"2017-01-01 10:00:00","date_modified":"2017-01-01 10:00:00","authors":[{"name":"bob jones","url":"http://example.com","avatar":"http://img.com/blah"}],"tags":["json","feed"],"attachments":[]}"#
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn deserialize_item__content_html() {
        let json = r#"{"id":"1","url":"http://example.com/feed.json","external_url":"http://example.com/feed.json","title":"feed title","content_html":"<p>content</p>","summary":"feed summary","image":"http://img.com/blah","banner_image":"http://img.com/blah","date_published":"2017-01-01 10:00:00","date_modified":"2017-01-01 10:00:00","authors":[{"name":"bob jones","url":"http://example.com","avatar":"http://img.com/blah"}],"tags":["json","feed"],"attachments":[]}"#;
        let item: Item = serde_json::from_str(&json).unwrap();
        let expected = Item {
            id: "1".into(),
            url: Some("http://example.com/feed.json".into()),
            external_url: Some("http://example.com/feed.json".into()),
            title: Some("feed title".into()),
            content_html: Some("<p>content</p>".into()),
            summary: Some("feed summary".into()),
            image: Some("http://img.com/blah".into()),
            banner_image: Some("http://img.com/blah".into()),
            date_published: Some("2017-01-01 10:00:00".into()),
            date_modified: Some("2017-01-01 10:00:00".into()),
            authors: Some(vec![Author::new().name("bob jones").url("http://example.com").avatar("http://img.com/blah")]),
            tags: Some(vec!["json".into(), "feed".into()]),
            attachments: Some(vec![]),
            ..Default::default()
        };
        assert_eq!(item, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn deserialize_item__content_text() {
        let json = r#"{"id":"1","url":"http://example.com/feed.json","external_url":"http://example.com/feed.json","title":"feed title","content_text":"content","summary":"feed summary","image":"http://img.com/blah","banner_image":"http://img.com/blah","date_published":"2017-01-01 10:00:00","date_modified":"2017-01-01 10:00:00","authors":[{"name":"bob jones","url":"http://example.com","avatar":"http://img.com/blah"}],"tags":["json","feed"],"attachments":[]}"#;
        let item: Item = serde_json::from_str(&json).unwrap();
        let expected = Item {
            id: "1".into(),
            url: Some("http://example.com/feed.json".into()),
            external_url: Some("http://example.com/feed.json".into()),
            title: Some("feed title".into()),
            content_text: Some("content".into()),
            summary: Some("feed summary".into()),
            image: Some("http://img.com/blah".into()),
            banner_image: Some("http://img.com/blah".into()),
            date_published: Some("2017-01-01 10:00:00".into()),
            date_modified: Some("2017-01-01 10:00:00".into()),
            authors: Some(vec![Author::new().name("bob jones").url("http://example.com").avatar("http://img.com/blah")]),
            tags: Some(vec!["json".into(), "feed".into()]),
            attachments: Some(vec![]),
            ..Default::default()
        };
        assert_eq!(item, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn deserialize_item__content_both() {
        let json = r#"{"id":"1","url":"http://example.com/feed.json","external_url":"http://example.com/feed.json","title":"feed title","content_html":"<p>content</p>","content_text":"content","summary":"feed summary","image":"http://img.com/blah","banner_image":"http://img.com/blah","date_published":"2017-01-01 10:00:00","date_modified":"2017-01-01 10:00:00","authors":[{"name":"bob jones","url":"http://example.com","avatar":"http://img.com/blah"}],"tags":["json","feed"],"attachments":[]}"#;
        let item: Item = serde_json::from_str(&json).unwrap();
        let expected = Item {
            id: "1".into(),
            url: Some("http://example.com/feed.json".into()),
            external_url: Some("http://example.com/feed.json".into()),
            title: Some("feed title".into()),
            content_html: Some("<p>content</p>".into()),
            content_text: Some("content".into()),
            summary: Some("feed summary".into()),
            image: Some("http://img.com/blah".into()),
            banner_image: Some("http://img.com/blah".into()),
            date_published: Some("2017-01-01 10:00:00".into()),
            date_modified: Some("2017-01-01 10:00:00".into()),
            authors: Some(vec![Author::new().name("bob jones").url("http://example.com").avatar("http://img.com/blah")]),
            tags: Some(vec!["json".into(), "feed".into()]),
            attachments: Some(vec![]),
            ..Default::default()
        };
        assert_eq!(item, expected);
    }
}

