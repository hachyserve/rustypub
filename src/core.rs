use chrono::{DateTime, Utc};

use crate::extended::{Actor, ActorBuilder};

pub trait Serialize
where
    Self: serde::Serialize,
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

/// Null-type object that implements `Serialize` for convenience
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Null {}

impl Serialize for Null {
    fn to_json(&self) -> String {
        self.to_json_pretty()
    }
    fn to_json_pretty(&self) -> String {
        panic!("intentionally unimplemented");
    }
    fn from_json(_json: String) -> Self {
        panic!("intentionally unimplemented");
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Document<T: Serialize> {
    #[serde(rename = "@context")]
    context: Context,

    #[serde(flatten)]
    object: T,
}

impl<T: Serialize> Serialize for Document<T> {
    fn from_json(_json: String) -> Self {
        Document {
            context: ContextBuilder::new().build(),
            // TODO: figure out how to know what type this is
            object: T::from_json(_json),
        }
    }
}

impl<T: Serialize> Document<T> {
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
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Context {
    #[serde(rename = "@vocab")]
    namespace: String,

    // TODO: figure out how to extend this per the above array/object options.
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
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Object<AttributedToT: Serialize> {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    object_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    published: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<Box<Link>>,

    #[serde(rename = "attributedTo", skip_serializing_if = "Vec::is_empty")]
    attributed_to: Vec<AttributedToT>,
}

#[derive(Clone)]
pub struct ObjectBuilder<AttributedToT: Serialize + Clone> {
    object_type: Option<String>,
    // TODO: actually an IRI: consider https://docs.rs/iref/latest/iref/
    id: Option<http::Uri>,
    name: Option<String>,
    url: Option<http::Uri>,
    published: Option<DateTime<Utc>>,
    image: Option<LinkBuilder>,
    attributed_to: Vec<AttributedToT>,
    // TODO: more fields
}

impl<AttributedToT: Serialize + Clone> ObjectBuilder<AttributedToT> {
    pub fn new() -> Self {
        ObjectBuilder {
            object_type: None,
            id: None,
            name: None,
            url: None,
            published: None,
            image: None,
            attributed_to: vec![],
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
        }
    }
}

impl<AttributedToT: Serialize + Clone> Serialize for Object<AttributedToT> {
    fn from_json(_json: String) -> Self {
        ObjectBuilder::new().build()
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Uri {
    href: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mediaType")]
    media_type: Option<String>,
}

impl Serialize for Uri {
    fn from_json(_json: String) -> Self {
        Uri {
            href: "todo".to_string(),
            media_type: None,
        }
    }
}

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

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Preview {
    #[serde(flatten)]
    base: Object<Null>,

    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Uri>,
}

impl Serialize for Preview {
    fn from_json(_json: String) -> Self {
        PreviewBuilder::new("todo".to_string(), "unimplemented".to_string()).build()
    }
}

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

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Link {
    #[serde(rename = "type")]
    link_type: String,

    #[serde(flatten)]
    href: Uri,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    rel: Vec<String>, // TODO: RFC5988 validation

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    hreflang: Option<String>, // TODO: BCP47 language tag

    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    preview: Option<Preview>,
}

impl Link {
    pub const TYPE: &'static str = "Link";
}

impl Serialize for Link {
    fn from_json(_json: String) -> Self {
        LinkBuilder::new(UriBuilder::new("href".parse::<http::Uri>().unwrap())).build()
    }
}

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

impl LinkBuilder {
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

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Activity {
    #[serde(flatten)]
    base: Object<Null>,

    summary: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    actor: Option<Actor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object: Option<Object<Null>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<Object<Null>>, // TODO: Target
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<String>, // TODO: Result
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<String>, // TODO: Origin
    #[serde(skip_serializing_if = "Option::is_none")]
    instrument: Option<String>, // TODO: Instrument
}

impl Serialize for Activity {
    fn from_json(_json: String) -> Self {
        ActivityBuilder::new("unknown".to_string(), "unimplemented".to_string()).build()
    }
}

pub struct ActivityBuilder {
    base: ObjectBuilder<Null>,
    summary: String,
    actor: Option<ActorBuilder>,
    object: Option<ObjectBuilder<Null>>,
    target: Option<ObjectBuilder<Null>>,
    result: Option<String>,
    origin: Option<String>,
    instrument: Option<String>,
}

impl ActivityBuilder {
    pub fn new(activity_type: String, summary: String) -> Self {
        ActivityBuilder {
            base: ObjectBuilder::new().object_type(activity_type),
            summary,
            actor: None,
            object: None,
            target: None,
            result: None,
            origin: None,
            instrument: None,
        }
    }

    pub fn published(mut self, datetime: DateTime<Utc>) -> Self {
        self.base.published(datetime);
        self
    }

    pub fn actor(mut self, actor: ActorBuilder) -> Self {
        self.actor = Some(actor);
        self
    }

    pub fn object(mut self, object: ObjectBuilder<Null>) -> Self {
        self.object = Some(object);
        self
    }

    pub fn target(mut self, target: ObjectBuilder<Null>) -> Self {
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

    pub fn build(self) -> Activity {
        Activity {
            base: self.base.build(),
            summary: self.summary,
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

#[cfg(test)]
mod tests {
    use crate::{core::*, extended::ActorBuilder};
    use pretty_assertions::assert_eq;

    #[test]
    fn create_activity_stream_object() {
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
        assert_eq!(actual.to_json_pretty(), expected)
    }

    #[test]
    fn create_link() {
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
        assert_eq!(actual.to_json_pretty(), expected)
    }

    #[test]
    fn create_preview() {
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
        assert_eq!(actual.to_json_pretty(), expected);
    }

    #[test]
    fn create_activity() {
        let actual = Document::new(
            ContextBuilder::new().build(),
            ActivityBuilder::new(
                "Activity".to_string(),
                "Sally did something to a note".to_string(),
            )
            .actor(ActorBuilder::new("Person".to_string()).name("Sally".to_string()))
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
        assert_eq!(actual.to_json_pretty(), expected);
    }
}
