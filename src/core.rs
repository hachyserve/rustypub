use http::Uri;
use serde::{Deserialize, Serialize};
use serde_tuple::*;

#[derive(Serialize_tuple, Deserialize_tuple, Debug)]
pub struct ActivityStreamsContext {
    pub namespace: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<ActivityStreamsContextLanguage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityStreamsContextLanguage {
    #[serde(rename = "@language")]
    pub language: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityStreamsObject {
    #[serde(rename = "@context")]
    pub context: ActivityStreamsContext,
    pub id: String,
    pub name: String,
}

impl ActivityStreamsObject {
    pub const NAMESPACE: &'static str = "https://www.w3.org/ns/activitystreams";
    pub const TYPE: &'static str = "Object";
    pub fn new(id: String, name: String) -> Self {
        return ActivityStreamsObject {
            context: ActivityStreamsContext {
                namespace: Self::NAMESPACE.to_string() + "#" + Self::TYPE,
                lang: Some(ActivityStreamsContextLanguage {
                    language: "en".to_string(),
                }),
            },
            id,
            name,
        };
    }

    // TODO: make this a trait
    pub fn to_json(&self) -> String {
        let serialized = serde_json::to_string(&self).unwrap();
        println!("serialized = {}", serialized);

        return serialized;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityStreamsUri {
    href: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mediaType")]
    media_type: Option<String>,
}

impl ActivityStreamsUri {
    pub fn to_json(&self) -> String {
        let serialized = serde_json::to_string(&self).unwrap();
        println!("serialized = {}", serialized);
        return serialized;
    }
}

pub struct ActivityStreamsUriBuilder {
    href: Uri,
    media_type: Option<String>,
}

impl ActivityStreamsUriBuilder {
    pub fn new(href: Uri) -> Self {
        ActivityStreamsUriBuilder {
            href,
            media_type: None,
        }
    }

    pub fn media_type(mut self, media_type: String) -> Self {
        self.media_type = Some(media_type);
        self
    }

    pub fn build(self) -> ActivityStreamsUri {
        ActivityStreamsUri {
            href: self.href.to_string(),
            media_type: self.media_type,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityStreamsPreview {
    #[serde(rename = "type")]
    preview_type: String,

    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<ActivityStreamsUri>,
}

impl ActivityStreamsPreview {
    pub fn to_json(&self) -> String {
        let serialized = serde_json::to_string(&self).unwrap();
        println!("serialized = {}", serialized);
        return serialized;
    }
}

pub struct ActivityStreamsPreviewBuilder {
    preview_type: String,
    name: String,
    duration: Option<String>,
    url: Option<ActivityStreamsUri>,
}

impl ActivityStreamsPreviewBuilder {
    pub fn new(preview_type: String, name: String) -> Self {
        ActivityStreamsPreviewBuilder {
            preview_type,
            name,
            duration: None,
            url: None,
        }
    }

    pub fn duration(mut self, dur: String) -> Self {
        self.duration = Some(dur);
        self
    }

    pub fn url(mut self, url: ActivityStreamsUri) -> Self {
        self.url = Some(url);
        self
    }

    pub fn build(self) -> ActivityStreamsPreview {
        ActivityStreamsPreview {
            preview_type: self.preview_type,
            name: self.name,
            duration: self.duration,
            url: self.url,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityStreamsLink {
    #[serde(rename = "@context")]
    context: ActivityStreamsContext,

    #[serde(flatten)]
    url: ActivityStreamsUri,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    rel: Vec<String>, // TODO: RFC5988 validation

    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    hreflang: Option<String>, // TODO: BCP47 language tag

    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    preview: Option<ActivityStreamsPreview>,
}

impl ActivityStreamsLink {
    pub const NAMESPACE: &'static str = "https://www.w3.org/ns/activitystreams";
    pub const TYPE: &'static str = "Link";

    pub fn to_json(&self) -> String {
        let serialized = serde_json::to_string(&self).unwrap();
        println!("serialized = {}", serialized);
        return serialized;
    }
}

pub struct ActivityStreamsLinkBuilder {
    context: ActivityStreamsContext,
    url: ActivityStreamsUri,
    rel: Vec<String>, // TODO: RFC5988 validation
    name: String,
    hreflang: Option<String>, // TODO: BCP47 language tag
    height: Option<u32>,
    width: Option<u32>,
    preview: Option<ActivityStreamsPreview>,
}

impl ActivityStreamsLinkBuilder {
    pub fn new(url: Uri, name: String) -> Self {
        ActivityStreamsLinkBuilder {
            context: ActivityStreamsContext {
                namespace: ActivityStreamsLink::NAMESPACE.to_string()
                    + "#"
                    + ActivityStreamsLink::TYPE,
                lang: None,
            },
            url: ActivityStreamsUriBuilder::new(url).build(),
            rel: Vec::new(),
            name,
            hreflang: None,
            height: None,
            width: None,
            preview: None,
        }
    }

    pub fn add_rel(mut self, rel: String) -> Self {
        self.rel.push(rel);
        self
    }

    pub fn hreflang(mut self, hreflang: String) -> Self {
        self.hreflang = Some(hreflang);
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn preview(mut self, preview: ActivityStreamsPreview) -> Self {
        self.preview = Some(preview);
        self
    }

    pub fn build(self) -> ActivityStreamsLink {
        ActivityStreamsLink {
            context: self.context,
            url: self.url,
            rel: self.rel,
            name: self.name,
            hreflang: self.hreflang,
            height: self.height,
            width: self.width,
            preview: self.preview,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{ActivityStreamsLinkBuilder, ActivityStreamsObject};
    use http::Uri;

    #[test]
    fn create_activity_stream_object() {
        let actual = ActivityStreamsObject::new("id".to_string(), "name".to_string());
        let expected = String::from(
            r#"{"@context":["https://www.w3.org/ns/activitystreams#Object",{"@language":"en"}],"id":"id","name":"name"}"#,
        );
        assert_eq!(actual.to_json(), expected)
    }

    #[test]
    fn create_link() {
        let actual = ActivityStreamsLinkBuilder::new(
            "http://example.org/abc".parse::<Uri>().unwrap(),
            "An example link".to_string(),
        )
        .hreflang("en".to_string())
        .build();
        let expected = String::from(
            r#"{"@context":["https://www.w3.org/ns/activitystreams#Link"],"href":"http://example.org/abc","name":"An example link","hreflang":"en"}"#,
        );
        assert_eq!(actual.to_json(), expected)
    }
}
