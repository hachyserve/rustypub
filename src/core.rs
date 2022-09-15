use crate::extended::{Actor, ActorBuilder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Result;

pub trait Serde<'de>
where
    Self: Serialize + Deserialize<'de>,
{
    fn to_json(&self) -> Result<String> {
        let serialized = serde_json::to_string(&self);
        println!("serialized = {:?}", serialized);
        serialized
    }

    fn to_json_pretty(&self) -> Result<String> {
        let serialized = serde_json::to_string_pretty(&self);
        println!("serialized = {:?}", serialized);
        serialized
    }

    fn from_json(json: &'de str) -> Result<Self> {
        serde_json::from_str(json)
    }
}

/// Null-type object that implements `Serde` for convenience
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Null {}

impl Serde<'_> for Null {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Document<T> {
    #[serde(rename = "@context")]
    pub context: Context,

    #[serde(flatten)]
    pub object: T,
}

impl<'a, T: Serde<'a>> Serde<'a> for Document<T> {}

impl<'a, T: Serde<'a>> Document<T> {
    pub fn new(context: Context, object: T) -> Self {
        Document { context, object }
    }
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Context {
    #[serde(rename = "@vocab")]
    namespace: String,

    #[serde(skip_serializing_if = "Option::is_none", rename = "@language")]
    language: Option<String>,
}

#[derive(Clone)]
pub struct ContextBuilder {
    namespace: String,
    language: Option<String>,
}

impl ContextBuilder {
    const NAMESPACE: &'static str = "https://www.w3.org/ns/activitystreams";

    pub fn new() -> Self {
        ContextBuilder {
            namespace: ContextBuilder::NAMESPACE.to_string(),
            language: None,
        }
    }

    // TODO: extend this to other options per the docs
    pub fn language(mut self, language: String) -> Self {
        self.language = Some(language);
        self
    }

    pub fn build(self) -> Context {
        Context {
            namespace: self.namespace,
            language: self.language,
        }
    }
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Object<AttributedToT> {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Box<Link>>,

    #[serde(
        rename = "attributedTo",
        skip_serializing_if = "Vec::is_empty",
        default = "Vec::new"
    )]
    pub attributed_to: Vec<AttributedToT>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<Box<Object<Null>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

#[derive(Clone)]
pub struct ObjectBuilder<AttributedToT> {
    object_type: Option<String>,
    // TODO: actually an IRI: consider https://docs.rs/iref/latest/iref/
    id: Option<http::Uri>,
    name: Option<String>,
    url: Option<http::Uri>,
    published: Option<DateTime<Utc>>,
    image: Option<LinkBuilder>,
    attributed_to: Vec<AttributedToT>,
    audience: Option<Box<ObjectBuilder<Null>>>,
    content: Option<String>,
    summary: Option<String>,
    // TODO: more fields
}

impl<'a, AttributedToT: Serde<'a> + Clone> ObjectBuilder<AttributedToT> {
    pub fn new() -> Self {
        ObjectBuilder {
            object_type: None,
            id: None,
            name: None,
            url: None,
            published: None,
            image: None,
            attributed_to: vec![],
            audience: None,
            content: None,
            summary: None,
        }
    }

    pub fn object_type(mut self, object_type: String) -> Self {
        self.object_type = Some(object_type);
        self
    }

    pub fn id(&mut self, id: http::Uri) -> Self {
        self.id = Some(id);
        self.clone()
    }

    pub fn name(&mut self, name: String) -> Self {
        self.name = Some(name);
        self.clone()
    }

    pub fn url(&mut self, url: http::Uri) -> Self {
        self.url = Some(url);
        self.clone()
    }

    pub fn published(&mut self, datetime: DateTime<Utc>) -> Self {
        self.published = Some(datetime);
        self.clone()
    }

    pub fn image(&mut self, image: LinkBuilder) -> Self {
        self.image = Some(image);
        self.clone()
    }

    pub fn add_attributed_to(mut self, attribution: AttributedToT) -> Self {
        self.attributed_to.push(attribution);
        self
    }

    pub fn audience(&mut self, audience: ObjectBuilder<Null>) -> Self {
        self.audience = Some(Box::new(audience));
        self.clone()
    }

    pub fn content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    pub fn summary(&mut self, summary: String) -> Self {
        self.summary = Some(summary);
        self.clone()
    }

    pub fn build(self) -> Object<AttributedToT> {
        Object {
            object_type: self.object_type,
            id: match self.id {
                None => None,
                uri => Some(uri.unwrap().to_string()),
            },
            name: self.name,
            url: match self.url {
                None => None,
                uri => Some(uri.unwrap().to_string()),
            },
            published: self.published,
            image: match self.image {
                None => None,
                i => Some(Box::new(i.unwrap().build())),
            },
            attributed_to: self.attributed_to,
            audience: match self.audience {
                None => None,
                a => Some(Box::new(a.unwrap().build())),
            },
            content: self.content,
            summary: self.summary,
        }
    }
}

impl<'a, AttributedToT: Serde<'a> + Clone> Serde<'a> for Object<AttributedToT> {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Uri {
    pub href: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mediaType")]
    pub media_type: Option<String>,
}

impl Serde<'_> for Uri {}

#[derive(Clone)]
pub struct UriBuilder {
    href: http::Uri,
    media_type: Option<String>,
}

impl UriBuilder {
    pub fn new(href: http::Uri) -> Self {
        UriBuilder {
            href,
            media_type: None,
        }
    }

    pub fn media_type(mut self, media_type: String) -> Self {
        self.media_type = Some(media_type);
        self
    }

    pub fn build(self) -> Uri {
        Uri {
            href: self.href.to_string(),
            media_type: self.media_type,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Preview {
    #[serde(flatten)]
    pub base: Object<Null>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Uri>,
}

impl Serde<'_> for Preview {}

pub struct PreviewBuilder {
    base: ObjectBuilder<Null>,
    duration: Option<String>,
    url: Option<Uri>,
}

impl PreviewBuilder {
    pub fn new(preview_type: String, name: String) -> Self {
        PreviewBuilder {
            base: ObjectBuilder::new().object_type(preview_type).name(name),
            duration: None,
            url: None,
        }
    }

    pub fn duration(mut self, dur: String) -> Self {
        self.duration = Some(dur);
        self
    }

    pub fn url(mut self, url: Uri) -> Self {
        self.url = Some(url);
        self
    }

    pub fn build(self) -> Preview {
        Preview {
            base: self.base.build(),
            duration: self.duration,
            url: self.url,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Link {
    #[serde(rename = "type")]
    pub link_type: String,

    #[serde(flatten)]
    pub href: Uri,

    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    pub rel: Vec<String>, // TODO: RFC5988 validation

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hreflang: Option<String>, // TODO: BCP47 language tag

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<Preview>,
}

impl Link {
    pub const TYPE: &'static str = "Link";
}

impl Serde<'_> for Link {}

#[derive(Clone)]
pub struct LinkBuilder {
    href: UriBuilder,
    rel: Vec<String>, // TODO: RFC5988 validation
    name: Option<String>,
    hreflang: Option<String>, // TODO: BCP47 language tag
    height: Option<u32>,
    width: Option<u32>,
    preview: Option<Preview>,
}

impl<'a> LinkBuilder {
    pub fn new(href: UriBuilder) -> Self {
        LinkBuilder {
            href,
            rel: Vec::new(),
            name: None,
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

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
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

    pub fn preview(mut self, preview: Preview) -> Self {
        self.preview = Some(preview);
        self
    }

    pub fn build(self) -> Link {
        Link {
            link_type: Link::TYPE.to_string(),
            href: self.href.build(),
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
pub struct Activity<'a> {
    // TODO: consider getters instead of raw access
    #[serde(flatten)]
    base: Object<Null>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<Actor<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<Object<Null>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<Object<Null>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>, // TODO: Origin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument: Option<String>, // TODO: Instrument
}

impl<'de: 'a, 'a> Serde<'de> for Activity<'a> {}

impl std::ops::Deref for Activity<'_> {
    type Target = Object<Null>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

#[derive(Clone)]
pub struct ActivityBuilder<'a> {
    base: ObjectBuilder<Null>,
    actor: Option<ActorBuilder<'a>>,
    object: Option<ObjectBuilder<Null>>,
    target: Option<ObjectBuilder<Null>>,
    result: Option<String>,
    origin: Option<String>,
    instrument: Option<String>,
}

impl<'a> ActivityBuilder<'a> {
    pub fn new(activity_type: String, summary: String) -> Self {
        ActivityBuilder {
            base: ObjectBuilder::new()
                .object_type(activity_type)
                .summary(summary),
            actor: None,
            object: None,
            target: None,
            result: None,
            origin: None,
            instrument: None,
        }
    }

    pub fn published(&mut self, datetime: DateTime<Utc>) -> Self {
        self.base.published(datetime);
        self.clone()
    }

    pub fn actor(&mut self, actor: ActorBuilder<'a>) -> Self {
        self.actor = Some(actor);
        self.clone()
    }

    pub fn object(&mut self, object: ObjectBuilder<Null>) -> Self {
        self.object = Some(object);
        self.clone()
    }

    pub fn target(&mut self, target: ObjectBuilder<Null>) -> Self {
        self.target = Some(target);
        self.clone()
    }

    pub fn result(&mut self, result: String) -> Self {
        self.result = Some(result);
        self.clone()
    }

    pub fn origin(&mut self, origin: String) -> Self {
        self.origin = Some(origin);
        self.clone()
    }

    pub fn instrument(&mut self, instrument: String) -> Self {
        self.instrument = Some(instrument);
        self.clone()
    }

    pub fn build(self) -> Activity<'a> {
        Activity {
            base: self.base.build(),
            actor: match self.actor {
                None => None,
                a => Some(a.unwrap().build()),
            },
            object: match self.object {
                None => None,
                o => Some(o.unwrap().build()),
            },
            target: match self.target {
                None => None,
                t => Some(t.unwrap().build()),
            },
            result: self.result,
            origin: self.origin,
            instrument: self.instrument,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IntransitiveActivity<'a> {
    #[serde(flatten)]
    base: Activity<'a>,
}

impl Serde<'_> for IntransitiveActivity<'_> {}

impl<'a> std::ops::Deref for IntransitiveActivity<'a> {
    type Target = Activity<'a>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

#[derive(Clone)]
pub struct IntransitiveActivityBuilder<'a> {
    base: ActivityBuilder<'a>,
}

impl<'a> IntransitiveActivityBuilder<'a> {
    pub fn new(activity_type: String, summary: String) -> Self {
        IntransitiveActivityBuilder {
            base: ActivityBuilder::new(activity_type, summary),
        }
    }

    pub fn published(mut self, datetime: DateTime<Utc>) -> Self {
        self.base.published(datetime);
        self
    }

    pub fn actor(mut self, actor: ActorBuilder<'a>) -> Self {
        self.base.actor(actor);
        self
    }

    pub fn target(mut self, target: ObjectBuilder<Null>) -> Self {
        self.base.target(target);
        self
    }

    pub fn result(mut self, result: String) -> Self {
        self.base.result(result);
        self
    }

    pub fn origin(mut self, origin: String) -> Self {
        self.base.origin(origin);
        self
    }

    pub fn instrument(mut self, instrument: String) -> Self {
        self.base.instrument(instrument);
        self
    }

    pub fn build(self) -> IntransitiveActivity<'a> {
        IntransitiveActivity {
            base: self.base.build(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{core::*, extended::ActorBuilder};
    use pretty_assertions::assert_eq;

    #[test]
    fn serialize_object() {
        let object: Object<Null> = ObjectBuilder::new().name("name".to_string()).build();
        let actual = Document::new(
            ContextBuilder::new().language("en".to_string()).build(),
            object,
        );
        let expected = String::from(
            r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams",
    "@language": "en"
  },
  "name": "name"
}"#,
        );
        assert!(actual.to_json_pretty().is_ok());
        assert_eq!(actual.to_json_pretty().unwrap(), expected)
    }

    #[test]
    fn deserialize_object() {
        let actual = String::from(
            r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams",
    "@language": "en"
  },
  "name": "name"
}"#,
        );
        let document: Document<Object<Null>> = Document::from_json(&actual).unwrap();
        assert_eq!(document.context.language, Some("en".to_string()));
        let object = document.object as Object<Null>;
        assert_eq!(object.name, Some("name".to_string()));
    }

    #[test]
    fn deserialize_object_malformed() {
        let actual = String::from(
            r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams",
    "@language": "en"
  },
}"#,
        );
        let result: Result<Document<Object<Null>>> = Document::from_json(&actual);
        assert!(result.is_err());
    }

    #[test]
    fn serialize_link() {
        let actual = Document::new(
            ContextBuilder::new().build(),
            LinkBuilder::new(UriBuilder::new(
                "http://example.org/abc".parse::<http::Uri>().unwrap(),
            ))
            .name("An example link".to_string())
            .hreflang("en".to_string())
            .build(),
        );
        let expected = String::from(
            r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "type": "Link",
  "href": "http://example.org/abc",
  "name": "An example link",
  "hreflang": "en"
}"#,
        );
        assert!(actual.to_json_pretty().is_ok());
        assert_eq!(actual.to_json_pretty().unwrap(), expected);
    }

    #[test]
    fn deserialize_link() {
        let actual = String::from(
            r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "type": "Link",
  "href": "http://example.org/abc",
  "name": "An example link",
  "hreflang": "en"
}"#,
        );
        let document: Document<Link> = Document::from_json(&actual).unwrap();
        let link = document.object as Link;
        assert_eq!(link.link_type, "Link");
        assert_eq!(link.href.href, "http://example.org/abc");
        assert_eq!(link.name, Some("An example link".to_string()));
        assert_eq!(link.hreflang, Some("en".to_string()));
    }

    #[test]
    fn serialize_preview() {
        let actual = Document::new(
            ContextBuilder::new().build(),
            PreviewBuilder::new("Video".to_string(), "Trailer".to_string())
                .duration("PT1M".to_string())
                .url(
                    UriBuilder::new(
                        "http://example.org/trailer.mkv"
                            .parse::<http::Uri>()
                            .unwrap(),
                    )
                    .media_type("video/mkv".to_string())
                    .build(),
                )
                .build(),
        );
        let expected = String::from(
            r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "type": "Video",
  "name": "Trailer",
  "duration": "PT1M",
  "url": {
    "href": "http://example.org/trailer.mkv",
    "mediaType": "video/mkv"
  }
}"#,
        );
        assert!(actual.to_json_pretty().is_ok());
        assert_eq!(actual.to_json_pretty().unwrap(), expected);
    }

    #[test]
    fn deserialize_preview() {
        let actual = String::from(
            r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "type": "Video",
  "name": "Trailer",
  "duration": "PT1M",
  "url": {
    "href": "http://example.org/trailer.mkv",
    "mediaType": "video/mkv"
  }
}"#,
        );
        let document: Document<Preview> = Document::from_json(&actual).unwrap();
        let preview = document.object as Preview;
        assert_eq!(preview.base.object_type, Some("Video".to_string()));
        assert_eq!(preview.base.name, Some("Trailer".to_string()));
        assert_eq!(preview.duration, Some("PT1M".to_string()));
        assert!(preview.url.is_some());
        assert_eq!(
            preview.url.as_ref().unwrap().href,
            "http://example.org/trailer.mkv".to_string()
        );
        assert_eq!(
            preview.url.as_ref().unwrap().media_type,
            Some("video/mkv".to_string())
        );
    }

    #[test]
    fn serialize_activity() {
        let actual = Document::new(
            ContextBuilder::new().build(),
            ActivityBuilder::new(
                "Activity".to_string(),
                "Sally did something to a note".to_string(),
            )
            .actor(ActorBuilder::new("Person").name("Sally"))
            .object(
                ObjectBuilder::new()
                    .object_type("Note".to_string())
                    .name("A Note".to_string()),
            )
            .build(),
        );

        let expected = String::from(
            r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "type": "Activity",
  "summary": "Sally did something to a note",
  "actor": {
    "type": "Person",
    "name": "Sally"
  },
  "object": {
    "type": "Note",
    "name": "A Note"
  }
}"#,
        );
        assert!(actual.to_json_pretty().is_ok());
        assert_eq!(actual.to_json_pretty().unwrap(), expected);
    }

    #[test]
    fn deserialize_activity() {
        let actual = String::from(
            r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "type": "Activity",
  "summary": "Sally did something to a note",
  "actor": {
    "type": "Person",
    "name": "Sally"
  },
  "object": {
    "type": "Note",
    "name": "A Note"
  }
}"#,
        );
        let document: Document<Activity> = Document::from_json(&actual).unwrap();
        let activity = document.object as Activity;
        assert_eq!(activity.object_type, Some("Activity".to_string()));
        assert_eq!(
            activity.summary,
            Some("Sally did something to a note".to_string())
        );

        assert!(activity.actor.is_some());
        let actor = activity.actor.as_ref().unwrap();
        assert_eq!(actor.object_type, Some("Person".to_string()));
        assert_eq!(actor.name, Some("Sally".to_string()));

        assert!(activity.object.is_some());
        let object = activity.object.as_ref().unwrap();
        assert_eq!(object.object_type, Some("Note".to_string()));
        assert_eq!(object.name, Some("A Note".to_string()));
    }
}
