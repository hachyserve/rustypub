use http::Uri;
use serde::{Deserialize, Serialize};
use serde_tuple::*;

use crate::extended::Actor;

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

/// JSON-LD uses the special @context property to define the processing context.
/// The value of the @context property is defined by the [JSON-LD]
/// specification. Implementations producing Activity Streams 2.0 documents
/// should include a @context property with a value that includes a reference to
/// the normative Activity Streams 2.0 JSON-LD @context definition using the URL
/// "https://www.w3.org/ns/activitystreams". Implementations may use the
/// alternative URL "http://www.w3.org/ns/activitystreams" instead. This can be
/// done using a string, object, or array.
/// https://www.w3.org/TR/activitystreams-core/#jsonld
#[derive(Serialize_tuple, Deserialize_tuple, Debug)]
pub struct ActivityStreamsContext {
    namespace: String,

    // TODO: figure out how to extend this per the above array/object options.
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<ActivityStreamsContextLanguage>,
}

pub struct ActivityStreamsContextBuilder {
    namespace: String,
    language: Option<ActivityStreamsContextLanguage>,
}

impl ActivityStreamsContextBuilder {
    const NAMESPACE: &'static str = "https://www.w3.org/ns/activitystreams";

    pub fn new() -> Self {
        ActivityStreamsContextBuilder {
            namespace: ActivityStreamsContextBuilder::NAMESPACE.to_string(),
            language: None,
        }
    }

    // TODO: extend this to other options per the docs
    pub fn language(mut self, language: String) -> Self {
        self.language = Some(ActivityStreamsContextLanguage { language });
        self
    }

    pub fn build(self) -> ActivityStreamsContext {
        ActivityStreamsContext {
            namespace: self.namespace,
            language: self.language,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityStreamsContextLanguage {
    #[serde(rename = "@language")]
    language: String,
}

/// The Object is the primary base type for the Activity Streams vocabulary.
/// In addition to having a global identifier (expressed as an absolute IRI
/// using the id property) and an "object type" (expressed using the type
/// property), all instances of the Object type share a common set of
/// properties normatively defined by the Activity Vocabulary. These
/// include: attachment | attributedTo | audience | content | context |
/// contentMap | name | nameMap | endTime | generator | icon | image |
/// inReplyTo | location | preview | published | replies | startTime |
/// summary | summaryMap | tag | updated | url | to | bto | cc | bcc |
/// mediaType | duration
/// All properties are optional (including the id and type).
#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityStreamsObject {
    #[serde(rename = "@context")]
    context: ActivityStreamsContext,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    object_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

pub struct ActivityStreamsObjectBuilder {
    context: ActivityStreamsContext,
    object_type: Option<String>,
    // TODO: actually an IRI: consider https://docs.rs/iref/latest/iref/
    id: Option<Uri>,
    name: Option<String>,
    // TODO: more fields
}

impl ActivityStreamsObjectBuilder {
    pub fn new() -> Self {
        ActivityStreamsObjectBuilder {
            context: ActivityStreamsContextBuilder::new().build(),
            object_type: None,
            id: None,
            name: None,
        }
    }

    pub fn new_with_language(language: String) -> Self {
        ActivityStreamsObjectBuilder {
            context: ActivityStreamsContextBuilder::new()
                .language(language)
                .build(),
            object_type: None,
            id: None,
            name: None,
        }
    }

    pub fn object_type(mut self, object_type: String) -> Self {
        self.object_type = Some(object_type);
        self
=======
    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn id(mut self, id: Uri) -> Self {
        self.id = Some(id);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn build(self) -> ActivityStreamsObject {
        ActivityStreamsObject {
            context: self.context,
            object_type: self.object_type,
            name: self.name,
            id: match self.id {
                None => None,
                uri => Some(uri.unwrap().to_string()),
            },
        }
    }
}

impl ActivityStreamsSerialize for ActivityStreamsObject {
    fn from_json(json: String) -> Self {
        ActivityStreamsObjectBuilder::new().build()
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
            base: ActivityStreamsObjectBuilder::new()
                .object_type(preview_type)
                .name(name)
                .build(),
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
            context: ActivityStreamsContextBuilder::new().build(),
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
        ActivityStreamsActivityBuilder::new("unimplemented".to_string()).build()
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
    pub fn new(summary: String) -> Self {
        ActivityStreamsActivityBuilder {
            base: ActivityStreamsObjectBuilder::new()
                .object_type(ActivityStreamsActivity::TYPE.to_string())
                .build(),
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
            base: self.base,
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
            ActivityStreamsLinkBuilder, ActivityStreamsObject, ActivityStreamsObjectBuilder,
            ActivityStreamsPreviewBuilder, ActivityStreamsSerialize, ActivityStreamsUriBuilder,
        },
        extended::ActorBuilder,
    };
    use http::Uri;

    use super::ActivityStreamsActivityBuilder;

    #[test]
    fn create_activity_stream_object() {
        let actual = ActivityStreamsObjectBuilder::new_with_language("en".to_string())
            .name("name".to_string())
            .build();
        let expected = String::from(
            r#"{
  "@context": [
    "https://www.w3.org/ns/activitystreams",
    {
      "@language": "en"
    }
  ],
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
            r#"{
  "@context": [
    "https://www.w3.org/ns/activitystreams"
  ],
  "type": "Link",
  "href": "http://example.org/abc",
  "name": "An example link",
  "hreflang": "en"
}"#,
        );
        assert_eq!(actual.to_json_pretty(), expected)
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
            r#"{
  "@context": [
    "https://www.w3.org/ns/activitystreams"
  ],
  "type": "Video",
  "name": "Trailer",
  "duration": "PT1M",
  "url": {
    "href": "http://example.org/trailer.mkv",
    "mediaType": "video/mkv"
  }
}"#,
        );
        assert_eq!(actual.to_json_pretty(), expected);
    }

    #[test]
    fn create_activity() {
        let actual =
            ActivityStreamsActivityBuilder::new("Sally did something to a note".to_string())
                .actor(
                    ActorBuilder::new(
                        ActivityStreamsObjectBuilder::new()
                            .object_type("Person".to_string())
                            .name("Sally".to_string())
                            .build(),
                    )
                    .build(),
                )
                .object(
                    ActivityStreamsObjectBuilder::new()
                        .object_type("Note".to_string())
                        .name("A Note".to_string())
                        .build(),
                )
                .build();

        let expected = String::from(
            r#"{
  "@context": [
    "https://www.w3.org/ns/activitystreams"
  ],
  "type": "Activity",
  "summary": "Sally did something to a note",
  "actor": {
    "@context": [
      "https://www.w3.org/ns/activitystreams"
    ],
    "type": "Person",
    "name": "Sally"
  },
  "object": {
    "@context": [
      "https://www.w3.org/ns/activitystreams"
    ],
    "type": "Note",
    "name": "A Note"
  }
}"#,
        );
        assert_eq!(actual.to_json_pretty(), expected);
    }
}
