use http::Uri;
use serde::{Deserialize, Serialize};
use serde_tuple::*;

use crate::extended::Actor;

const NAMESPACE: &str = "https://www.w3.org/ns/activitystreams";

pub trait ActivityStreamsSerialize
where
    Self: Serialize,
{
    fn to_json(&self) -> String {
        let serialized = serde_json::to_string(&self).unwrap();
        println!("serialized = {}", serialized);
        serialized
    }

    fn to_json_pretty(&self) -> String {
        let serialized = serde_json::to_string_pretty(&self).unwrap();
        println!("serialized = {}", serialized);
        serialized
    }

    fn from_json(json: String) -> Self;
}

#[derive(Serialize_tuple, Deserialize_tuple, Debug)]
pub struct ActivityStreamsContext {
    namespace: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<ActivityStreamsContextLanguage>,
}

impl ActivityStreamsContext {
    pub fn new() -> Self {
        ActivityStreamsContext {
            namespace: NAMESPACE.to_string(),
            language: None,
        }
    }

    pub fn language(mut self, language: ActivityStreamsContextLanguage) -> Self {
        self.language = Some(language);
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityStreamsContextLanguage {
    #[serde(rename = "@language")]
    language: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityStreamsObject {
    #[serde(rename = "@context")]
    context: ActivityStreamsContext,
    #[serde(rename = "type")]
    object_type: String,
    id: String,
    name: String,
}

impl ActivityStreamsObject {
    pub fn new(id: String, name: String) -> Self {
        ActivityStreamsObject {
            context: ActivityStreamsContext::new(),
            object_type: "Object".to_string(),
            id,
            name,
        }
    }

    pub fn language(mut self, lang: String) -> Self {
        self.context = self
            .context
            .language(ActivityStreamsContextLanguage { language: lang });
        self
    }

    pub fn object_type(mut self, object_type: String) -> Self {
        self.object_type = object_type;
        self
    }
}

impl ActivityStreamsSerialize for ActivityStreamsObject {
    fn from_json(json: String) -> Self {
        ActivityStreamsObject::new("todo".to_string(), "unimplemented".to_string())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityStreamsUri {
    href: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mediaType")]
    media_type: Option<String>,
}

impl ActivityStreamsSerialize for ActivityStreamsUri {
    fn from_json(json: String) -> Self {
        ActivityStreamsUri {
            href: "todo".to_string(),
            media_type: None,
        }
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
    #[serde(flatten)]
    base: ActivityStreamsObject,

    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<ActivityStreamsUri>,
}

impl ActivityStreamsSerialize for ActivityStreamsPreview {
    fn from_json(json: String) -> Self {
        ActivityStreamsPreviewBuilder::new("todo".to_string(), "unimplemented".to_string()).build()
    }
}

pub struct ActivityStreamsPreviewBuilder {
    base: ActivityStreamsObject,
    duration: Option<String>,
    url: Option<ActivityStreamsUri>,
}

impl ActivityStreamsPreviewBuilder {
    pub fn new(preview_type: String, name: String) -> Self {
        ActivityStreamsPreviewBuilder {
            base: ActivityStreamsObject::new("todo_id".to_string(), name).object_type(preview_type),
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
            base: self.base,
            duration: self.duration,
            url: self.url,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityStreamsLink {
    #[serde(rename = "@context")]
    context: ActivityStreamsContext,

    #[serde(rename = "type")]
    link_type: String,

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
    pub const TYPE: &'static str = "Link";
}

impl ActivityStreamsSerialize for ActivityStreamsLink {
    fn from_json(json: String) -> Self {
        ActivityStreamsLinkBuilder::new("todo".parse::<Uri>().unwrap(), "unimplemented".to_string())
            .build()
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
            context: ActivityStreamsContext::new(),
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
            link_type: ActivityStreamsLink::TYPE.to_string(),
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityStreamsActivity {
    #[serde(flatten)]
    base: ActivityStreamsObject,

    summary: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    actor: Option<Actor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object: Option<ActivityStreamsObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<String>, // TODO: ActivityStreamsTarget
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<String>, // TODO: ActivityStreamsResult
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<String>, // TODO: ActivityStreamsOrigin
    #[serde(skip_serializing_if = "Option::is_none")]
    instrument: Option<String>, // TODO: ActivityStreamsInstrument
}

impl ActivityStreamsActivity {
    pub const TYPE: &'static str = "Activity";
}

impl ActivityStreamsSerialize for ActivityStreamsActivity {
    fn from_json(json: String) -> Self {
        ActivityStreamsActivityBuilder::new(
            ActivityStreamsObject::new("todo".to_string(), "unimplemented".to_string())
                .object_type(ActivityStreamsActivity::TYPE.to_string()),
            "not implemented yet".to_string(),
        )
        .build()
    }
}

pub struct ActivityStreamsActivityBuilder {
    base: ActivityStreamsObject,
    summary: String,
    actor: Option<Actor>,
    object: Option<ActivityStreamsObject>,
    target: Option<String>,
    result: Option<String>,
    origin: Option<String>,
    instrument: Option<String>,
}

impl ActivityStreamsActivityBuilder {
    pub fn new(base: ActivityStreamsObject, summary: String) -> Self {
        ActivityStreamsActivityBuilder {
            base,
            summary,
            actor: None,
            object: None,
            target: None,
            result: None,
            origin: None,
            instrument: None,
        }
    }

    pub fn actor(mut self, actor: Actor) -> Self {
        self.actor = Some(actor);
        self
    }

    pub fn object(mut self, object: ActivityStreamsObject) -> Self {
        self.object = Some(object);
        self
    }

    pub fn target(mut self, target: String) -> Self {
        self.target = Some(target);
        self
    }

    pub fn result(mut self, result: String) -> Self {
        self.result = Some(result);
        self
    }

    pub fn origin(mut self, origin: String) -> Self {
        self.origin = Some(origin);
        self
    }

    pub fn instrument(mut self, instrument: String) -> Self {
        self.instrument = Some(instrument);
        self
    }

    pub fn build(self) -> ActivityStreamsActivity {
        ActivityStreamsActivity {
            base: self
                .base
                .object_type(ActivityStreamsActivity::TYPE.to_string()),
            summary: self.summary,
            actor: self.actor,
            object: self.object,
            target: self.target,
            result: self.result,
            origin: self.origin,
            instrument: self.instrument,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::{
            ActivityStreamsLinkBuilder, ActivityStreamsObject, ActivityStreamsPreviewBuilder,
            ActivityStreamsSerialize, ActivityStreamsUriBuilder,
        },
        extended::ActorBuilder,
    };
    use http::Uri;

    use super::ActivityStreamsActivityBuilder;

    #[test]
    fn create_activity_stream_object() {
        let actual = ActivityStreamsObject::new("id".to_string(), "name".to_string())
            .language("en".to_string());
        let expected = String::from(
            r#"{
  "@context": [
    "https://www.w3.org/ns/activitystreams",
    {
      "@language": "en"
    }
  ],
  "type": "Object",
  "id": "id",
  "name": "name"
}"#,
        );
        assert_eq!(actual.to_json_pretty(), expected)
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
            r#"{"@context":["https://www.w3.org/ns/activitystreams"],"type":"Link","href":"http://example.org/abc","name":"An example link","hreflang":"en"}"#,
        );
        assert_eq!(actual.to_json(), expected)
    }

    #[test]
    fn create_preview() {
        let actual = ActivityStreamsPreviewBuilder::new("Video".to_string(), "Trailer".to_string())
            .duration("PT1M".to_string())
            .url(
                ActivityStreamsUriBuilder::new(
                    "http://example.org/trailer.mkv".parse::<Uri>().unwrap(),
                )
                .media_type("video/mkv".to_string())
                .build(),
            )
            .build();
        let expected = String::from(
            r#"{"@context":["https://www.w3.org/ns/activitystreams"],"type":"Video","id":"todo_id","name":"Trailer","duration":"PT1M","url":{"href":"http://example.org/trailer.mkv","mediaType":"video/mkv"}}"#,
        );
        assert_eq!(actual.to_json(), expected);
    }

    #[test]
    fn create_activity() {
        let actual = ActivityStreamsActivityBuilder::new(
            ActivityStreamsObject::new("id".to_string(), "name".to_string()),
            "Sally did something to a note".to_string(),
        )
        .actor(
            ActorBuilder::new(
                "Person".to_string(),
                "sally".to_string(),
                "Sally".to_string(),
            )
            .build(),
        )
        .object(
            ActivityStreamsObject::new("note".to_string(), "A Note".to_string())
                .object_type("Note".to_string()),
        )
        .build();

        let expected = String::from(
            r#"{"@context":["https://www.w3.org/ns/activitystreams"],"type":"Activity","id":"id","name":"name","summary":"Sally did something to a note","actor":{"@context":["https://www.w3.org/ns/activitystreams"],"type":"Person","id":"sally","name":"Sally"},"object":{"@context":["https://www.w3.org/ns/activitystreams"],"type":"Note","id":"note","name":"A Note"}}"#,
        );
        assert_eq!(actual.to_json(), expected);
    }
}
