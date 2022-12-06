use crate::extended::{Actor, ActorBuilder};
use crate::Serde;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// [Null]-type object that implements [Serde] for convenience
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Null {}

// TODO: create a derive macro for this
impl Serde for Null {}

// TODO: rename to something else as there's a [Document] in the Activity
// Streams spec.
/// Outer object for serialization and deserialization. Not an Activity Streams
/// 2.0 object.
#[derive(Serialize, Deserialize, Debug)]
pub struct Document<T> {
    #[serde(rename = "@context")]
    pub context: Context,

    #[serde(flatten)]
    pub object: T,
}

impl<T> Serde for Document<T> where T: Serde {}

impl<T: Serde> Document<T> {
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
/// <https://www.w3.org/TR/activitystreams-core/#jsonld>
#[derive(Serialize, Deserialize, Debug)]
pub struct Context {
    #[serde(rename = "@vocab")]
    namespace: String,

    #[serde(skip_serializing_if = "Option::is_none", rename = "@language")]
    language: Option<String>,
}

/// Builder struct for [Context].
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

impl Default for ContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// The [Object] is the primary base type for the Activity Streams vocabulary.
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

impl<AttributedToT> Serde for Object<AttributedToT> where AttributedToT: Serde + Clone {}

/// Builder for [Object].
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

impl<AttributedToT: Serde + Clone> ObjectBuilder<AttributedToT> {
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
            id: self.id.map(|uri| uri.to_string()),
            name: self.name,
            url: self.url.map(|uri| uri.to_string()),
            published: self.published,
            image: self.image.map(|i| Box::new(i.build())),
            attributed_to: self.attributed_to,
            audience: self.audience.map(|a| Box::new(a.build())),
            content: self.content,
            summary: self.summary,
        }
    }
}

impl<AttributedToT: Serde + Clone> Default for ObjectBuilder<AttributedToT> {
    fn default() -> Self {
        Self::new()
    }
}

/// A utility struct to describe a URI.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Uri {
    pub href: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mediaType")]
    pub media_type: Option<String>,
}

impl Serde for Uri {}

/// Builder struct for [Uri].
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

/// Identifies an entity that provides a preview of this object.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Preview {
    #[serde(flatten)]
    pub base: Object<Null>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Uri>,
}

impl Serde for Preview {}

/// Builder for [Preview].
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

/// A [Link] is an indirect, qualified reference to a resource identified by a
/// URL. The fundamental model for links is established by
/// [RFC5988](https://www.w3.org/TR/activitystreams-vocabulary/#bib-RFC5988).
/// Many of the properties defined by the Activity Vocabulary allow values that
/// are either instances of [Object] or [Link]. When a [Link] is used, it
/// establishes a qualified relation connecting the subject (the containing
/// object) to the resource identified by the href. Properties of the [Link]
/// are properties of the reference as opposed to properties of the resource.
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

impl Serde for Link {}

/// Builder for a [Link] struct.
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

/// An [Activity] is a subtype of [Object] that describes some form of action
/// that may happen, is currently happening, or has already happened. The
/// [Activity] type itself serves as an abstract base type for all types of
/// activities. It is important to note that the [Activity] type itself does
/// not carry any specific semantics about the kind of action being taken.
#[derive(Serialize, Deserialize, Debug)]
pub struct Activity {
    #[serde(flatten)]
    base: Object<Null>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<Actor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<Object<Null>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<Object<Null>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>, // TODO: Origin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument: Option<String>, // TODO: Instrument
}

impl Serde for Activity {}

impl std::ops::Deref for Activity {
    type Target = Object<Null>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

/// Builder for an [Activity].
#[derive(Clone)]
pub struct ActivityBuilder {
    base: ObjectBuilder<Null>,
    actor: Option<ActorBuilder>,
    object: Option<ObjectBuilder<Null>>,
    target: Option<ObjectBuilder<Null>>,
    result: Option<String>,
    to: Option<Vec<String>>,
    origin: Option<String>,
    instrument: Option<String>,
}

impl ActivityBuilder {
    pub fn new(activity_type: String, summary: String) -> Self {
        ActivityBuilder {
            base: ObjectBuilder::new()
                .object_type(activity_type)
                .summary(summary),
            actor: None,
            object: None,
            target: None,
            result: None,
            to: None,
            origin: None,
            instrument: None,
        }
    }

    pub fn id(mut self, id: http::Uri) -> Self {
        self.base.id(id);
        self
    }

    pub fn published(&mut self, datetime: DateTime<Utc>) -> Self {
        self.base.published(datetime);
        self.clone()
    }

    pub fn actor(&mut self, actor: ActorBuilder) -> Self {
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

    pub fn to(&mut self, to: Vec<String>) -> Self {
        self.to = Some(to.clone());
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

    pub fn build(self) -> Activity {
        Activity {
            base: self.base.build(),
            actor: self.actor.map(|a| a.build()),
            object: self.object.map(|o| o.build()),
            target: self.target.map(|t| t.build()),
            result: self.result,
            to: self.to,
            origin: self.origin,
            instrument: self.instrument,
        }
    }
}

/// Instances of [IntransitiveActivity] are a subtype of [Activity] representing
/// intransitive actions. The object property is therefore inappropriate for
/// these activities.
#[derive(Serialize, Deserialize, Debug)]
pub struct IntransitiveActivity {
    #[serde(flatten)]
    base: Activity,
}

impl Serde for IntransitiveActivity {}

impl std::ops::Deref for IntransitiveActivity {
    type Target = Activity;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

/// Builder for an [IntransitiveActivity].
#[derive(Clone)]
pub struct IntransitiveActivityBuilder {
    base: ActivityBuilder,
}

impl IntransitiveActivityBuilder {
    pub fn new(activity_type: String, summary: String) -> Self {
        IntransitiveActivityBuilder {
            base: ActivityBuilder::new(activity_type, summary),
        }
    }

    pub fn published(mut self, datetime: DateTime<Utc>) -> Self {
        self.base.published(datetime);
        self
    }

    pub fn actor(mut self, actor: ActorBuilder) -> Self {
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

    pub fn build(self) -> IntransitiveActivity {
        IntransitiveActivity {
            base: self.base.build(),
        }
    }
}

/// A [Collection] is a subtype of [Object] that represents ordered or unordered
/// sets of [Object] or [Link] instances. Refer to the Activity Streams 2.0 Core
/// specification for a complete description of the [Collection] type.
#[derive(Serialize, Deserialize, Debug)]
pub struct Collection<CollectionT> {
    #[serde(flatten)]
    base: Object<Null>,

    #[serde(rename = "totalItems", skip_serializing_if = "Option::is_none")]
    pub total_items: Option<usize>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CollectionT>,
}

impl<CollectionT> Serde for Collection<CollectionT> where CollectionT: Serde {}

impl<CollectionT> std::ops::Deref for Collection<CollectionT>
where
    CollectionT: Serde,
{
    type Target = Object<Null>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

/// Builder for a [Collection].
pub struct CollectionBuilder<CollectionT>
where
    CollectionT: Serde,
{
    base: ObjectBuilder<Null>,
    items: Vec<CollectionT>,
}

impl<CollectionT> CollectionBuilder<CollectionT>
where
    CollectionT: Serde,
{
    pub fn new(collection_type: String, items: Vec<CollectionT>) -> Self {
        CollectionBuilder {
            base: ObjectBuilder::new().object_type(collection_type),
            items,
        }
    }

    pub fn build(self) -> Collection<CollectionT> {
        Collection {
            base: self.base.build(),
            total_items: match self.items.is_empty() {
                true => None,
                false => Some(self.items.len()),
            },
            items: self.items,
        }
    }
}

/// A subtype of [Collection] in which members of the logical collection are
/// assumed to always be strictly ordered.
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderedCollection<CollectionT> {
    #[serde(flatten)]
    base: Object<Null>,

    #[serde(rename = "totalItems", skip_serializing_if = "Option::is_none")]
    pub total_items: Option<usize>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "orderedItems")]
    pub ordered_items: Vec<CollectionT>,
}

impl<CollectionT> Serde for OrderedCollection<CollectionT> where CollectionT: Serde {}

impl<CollectionT> std::ops::Deref for OrderedCollection<CollectionT>
where
    CollectionT: Serde,
{
    type Target = Object<Null>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

/// Builder for an [OrderedCollection].
pub struct OrderedCollectionBuilder<CollectionT>
where
    CollectionT: Serde,
{
    base: ObjectBuilder<Null>,
    ordered_items: Vec<CollectionT>,
}

impl<CollectionT> OrderedCollectionBuilder<CollectionT>
where
    CollectionT: Serde,
{
    pub fn new(collection_type: String, ordered_items: Vec<CollectionT>) -> Self {
        OrderedCollectionBuilder {
            base: ObjectBuilder::new().object_type(collection_type),
            ordered_items,
        }
    }

    pub fn build(self) -> OrderedCollection<CollectionT> {
        OrderedCollection {
            base: self.base.build(),
            total_items: match self.ordered_items.is_empty() {
                true => None,
                false => Some(self.ordered_items.len()),
            },
            ordered_items: self.ordered_items,
        }
    }
}

/// Used to represent distinct subsets of items from a [Collection]. Refer to
/// the Activity Streams 2.0 Core for a complete description of the
/// [CollectionPage] object.
#[derive(Serialize, Deserialize, Debug)]
pub struct CollectionPage<CollectionT> {
    #[serde(flatten)]
    base: Collection<CollectionT>,

    #[serde(rename = "partOf")]
    pub part_of: String,

    pub next: Option<String>,

    pub prev: Option<String>,
}

impl<CollectionT> Serde for CollectionPage<CollectionT> where CollectionT: Serde {}

impl<CollectionT> std::ops::Deref for CollectionPage<CollectionT>
where
    CollectionT: Serde,
{
    type Target = Collection<CollectionT>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

/// Builder for a [CollectionPage].
pub struct CollectionPageBuilder<CollectionT>
where
    CollectionT: Serde,
{
    base: CollectionBuilder<CollectionT>,
    part_of: http::Uri,
    next: Option<http::Uri>,
    prev: Option<http::Uri>,
}

impl<CollectionT> CollectionPageBuilder<CollectionT>
where
    CollectionT: Serde,
{
    pub fn new(collection_type: String, items: Vec<CollectionT>, part_of: http::Uri) -> Self {
        CollectionPageBuilder {
            base: CollectionBuilder::new(collection_type, items),
            part_of,
            next: None,
            prev: None,
        }
    }

    pub fn next(mut self, next: http::Uri) -> Self {
        self.next = Some(next);
        self
    }

    pub fn prev(mut self, prev: http::Uri) -> Self {
        self.prev = Some(prev);
        self
    }

    pub fn build(self) -> CollectionPage<CollectionT> {
        CollectionPage {
            base: self.base.build(),
            part_of: self.part_of.to_string(),
            next: self.next.map(|n| n.to_string()),
            prev: self.prev.map(|p| p.to_string()),
        }
    }
}

/// Used to represent ordered subsets of items from an [OrderedCollection].
/// Refer to the Activity Streams 2.0 Core for a complete description of
/// the [OrderedCollectionPage] object.
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderedCollectionPage<CollectionT> {
    #[serde(flatten)]
    base: OrderedCollection<CollectionT>,

    #[serde(rename = "partOf")]
    pub part_of: String,

    pub next: Option<String>,

    pub prev: Option<String>,
}

impl<CollectionT> Serde for OrderedCollectionPage<CollectionT> where CollectionT: Serde {}

impl<CollectionT> std::ops::Deref for OrderedCollectionPage<CollectionT>
where
    CollectionT: Serde,
{
    type Target = OrderedCollection<CollectionT>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

/// Builder for a [OrderedCollectionPage].
pub struct OrderedCollectionPageBuilder<CollectionT>
where
    CollectionT: Serde,
{
    base: OrderedCollectionBuilder<CollectionT>,
    part_of: http::Uri,
    next: Option<http::Uri>,
    prev: Option<http::Uri>,
}

impl<CollectionT> OrderedCollectionPageBuilder<CollectionT>
where
    CollectionT: Serde,
{
    pub fn new(collection_type: String, items: Vec<CollectionT>, part_of: http::Uri) -> Self {
        OrderedCollectionPageBuilder {
            base: OrderedCollectionBuilder::new(collection_type, items),
            part_of,
            next: None,
            prev: None,
        }
    }

    pub fn next(mut self, next: http::Uri) -> Self {
        self.next = Some(next);
        self
    }

    pub fn prev(mut self, prev: http::Uri) -> Self {
        self.prev = Some(prev);
        self
    }

    pub fn build(self) -> OrderedCollectionPage<CollectionT> {
        OrderedCollectionPage {
            base: self.base.build(),
            part_of: self.part_of.to_string(),
            next: self.next.map(|n| n.to_string()),
            prev: self.prev.map(|p| p.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::Result;

    #[test]
    fn serialize_object() {
        let object: Object<Null> = ObjectBuilder::new().name(String::from("name")).build();
        let actual = Document::new(
            ContextBuilder::new().language(String::from("en")).build(),
            object,
        );
        let expected = r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams",
    "@language": "en"
  },
  "name": "name"
}"#;
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
        let document: Document<Object<Null>> = Document::from_json(actual).unwrap();
        assert_eq!(document.context.language, Some(String::from("en")));
        let object = document.object as Object<Null>;
        assert_eq!(object.name, Some(String::from("name")));
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
        let result: Result<Document<Object<Null>>> = Document::from_json(actual);
        assert!(result.is_err());
    }

    #[test]
    fn serialize_link() {
        let actual = Document::new(
            ContextBuilder::new().build(),
            LinkBuilder::new(UriBuilder::new(
                "http://example.org/abc".parse::<http::Uri>().unwrap(),
            ))
            .name(String::from("An example link"))
            .hreflang(String::from("en"))
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
        let document: Document<Link> = Document::from_json(actual).unwrap();
        let link = document.object as Link;
        assert_eq!(link.link_type, "Link");
        assert_eq!(link.href.href, "http://example.org/abc");
        assert_eq!(link.name, Some(String::from("An example link")));
        assert_eq!(link.hreflang, Some(String::from("en")));
    }

    #[test]
    fn serialize_preview() {
        let actual = Document::new(
            ContextBuilder::new().build(),
            PreviewBuilder::new(String::from("Video"), String::from("Trailer"))
                .duration(String::from("PT1M"))
                .url(
                    UriBuilder::new(
                        "http://example.org/trailer.mkv"
                            .parse::<http::Uri>()
                            .unwrap(),
                    )
                    .media_type(String::from("video/mkv"))
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
        let document: Document<Preview> = Document::from_json(actual).unwrap();
        let preview = document.object as Preview;
        assert_eq!(preview.base.object_type, Some(String::from("Video")));
        assert_eq!(preview.base.name, Some(String::from("Trailer")));
        assert_eq!(preview.duration, Some(String::from("PT1M")));
        assert!(preview.url.is_some());
        assert_eq!(
            preview.url.as_ref().unwrap().href,
            "http://example.org/trailer.mkv".to_string()
        );
        assert_eq!(
            preview.url.as_ref().unwrap().media_type,
            Some(String::from("video/mkv"))
        );
    }

    #[test]
    fn serialize_activity() {
        let actual = Document::new(
            ContextBuilder::new().build(),
            ActivityBuilder::new(
                String::from("Activity"),
                String::from("Sally did something to a note"),
            )
            .actor(ActorBuilder::new(String::from("Person")).name(String::from("Sally")))
            .object(
                ObjectBuilder::new()
                    .object_type(String::from("Note"))
                    .name(String::from("A Note")),
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
        let document: Document<Activity> = Document::from_json(actual).unwrap();
        let activity = document.object as Activity;
        assert_eq!(activity.object_type, Some(String::from("Activity")));
        assert_eq!(
            activity.summary,
            Some(String::from("Sally did something to a note"))
        );

        assert!(activity.actor.is_some());
        let actor = activity.actor.as_ref().unwrap();
        assert_eq!(actor.object_type, Some(String::from("Person")));
        assert_eq!(actor.name, Some(String::from("Sally")));

        assert!(activity.object.is_some());
        let object = activity.object.as_ref().unwrap();
        assert_eq!(object.object_type, Some(String::from("Note")));
        assert_eq!(object.name, Some(String::from("A Note")));
    }
}
