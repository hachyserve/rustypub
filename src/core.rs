use http::Uri;
use serde::{Deserialize, Serialize};
use serde_tuple::*;

#[derive(Serialize_tuple, Deserialize_tuple, Debug)]
pub struct ActivityStreamContext {
    pub namespace: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<ActivityStreamContextLanguage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityStreamContextLanguage {
    #[serde(rename = "@language")]
    pub language: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityStreamsObject {
    #[serde(rename = "@context")]
    pub context: ActivityStreamContext,
    pub id: String,
    pub name: String,
}

impl ActivityStreamsObject {
    pub const NAMESPACE: &'static str = "https://www.w3.org/ns/activitystreams";
    pub const TYPE: &'static str = "Object";
    pub fn new(id: String, name: String) -> Self {
        return ActivityStreamsObject {
            context: ActivityStreamContext {
                namespace: Self::NAMESPACE.to_string() + "#" + Self::TYPE,
                lang: Some(ActivityStreamContextLanguage {
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
    pub media_type: Option<String>,
}

impl ActivityStreamsUri {
    pub fn new(uri: Uri) -> Self {
        return ActivityStreamsUri {
            href: uri.to_string(),
            media_type: None,
        };
    }

    pub fn to_json(&self) -> String {
        let serialized = serde_json::to_string(&self).unwrap();
        println!("serialized = {}", serialized);
        return serialized;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityStreamsPreview {
    #[serde(rename = "type")]
    pub preview_type: String,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<ActivityStreamsUri>,
}

impl ActivityStreamsPreview {
    pub fn new(preview_type: String, name: String) -> Self {
        return ActivityStreamsPreview {
            preview_type,
            name,
            duration: None,
            url: None,
        };
    }

    pub fn to_json(&self) -> String {
        let serialized = serde_json::to_string(&self).unwrap();
        println!("serialized = {}", serialized);
        return serialized;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityStreamsLink {
    #[serde(rename = "@context")]
    pub context: ActivityStreamContext,

    #[serde(flatten)]
    uri: ActivityStreamsUri,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rel: Vec<String>, // TODO: RFC5988 validation

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hreflang: Option<String>, // TODO: BCP47 language tag

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<ActivityStreamsPreview>,
}

impl ActivityStreamsLink {
    pub const NAMESPACE: &'static str = "https://www.w3.org/ns/activitystreams";
    pub const TYPE: &'static str = "Link";
    pub fn new(uri: Uri, name: String) -> Self {
        return ActivityStreamsLink {
            context: ActivityStreamContext {
                namespace: Self::NAMESPACE.to_string() + "#" + Self::TYPE,
                lang: None,
            },
            uri: ActivityStreamsUri::new(uri),
            rel: Vec::new(),
            name,
            hreflang: None,
            height: None,
            width: None,
            preview: None,
        };
    }

    pub fn to_json(&self) -> String {
        let serialized = serde_json::to_string(&self).unwrap();
        println!("serialized = {}", serialized);
        return serialized;
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{ActivityStreamsLink, ActivityStreamsObject};
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
        let actual = ActivityStreamsLink::new(
            "http://example.org/abc".parse::<Uri>().unwrap(),
            "An example link".to_string(),
        );
        let expected = String::from(
            r#"{"@context":["https://www.w3.org/ns/activitystreams#Link"],"href":"http://example.org/abc","name":"An example link"}"#,
        );
        assert_eq!(actual.to_json(), expected)
    }
}
