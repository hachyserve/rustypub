pub mod object;
pub mod activity;
pub mod actor;

pub use object::*;

use serde::{de::DeserializeOwned, Deserializer, Deserialize, Serialize};
use derive_builder::Builder;
use object::{ Object, ObjectBuilder };

// TODO: rename to something else as there's a [Document] in the Activity
// Streams spec.
/// Outer object for serialization and deserialization. Not an Activity Streams
/// 2.0 object.
#[derive(Serialize, Deserialize, Debug, Clone, Builder)]
pub struct Document<T> where {
    #[serde(rename = "@context", deserialize_with = "context_deserializer")]
    pub context: Context,

    #[serde(flatten)]
    pub object: T,
}

impl<T : DeserializeOwned + Serialize > Document<T> {
    pub fn new(context: Context, object: T) -> Self {
        Document { context, object }
    }

    pub fn pretty_print(&self) -> serde_json::Result<String>  {
        let serialized = serde_json::to_string_pretty(&self);
        println!("serialized = {:?}", serialized);
        serialized
    }
}

impl<T: DeserializeOwned + Serialize> Document<T> {
    pub fn deserialize_string(json: String) -> serde_json::Result<Document<T>> {
        serde_json::from_str(json.as_str())
    }
}

///////////////////////////
// Context 
///////////////////////////
/// JSON-LD uses the special @context property to define the processing context.
/// The value of the @context property is defined by the [JSON-LD]
/// specification. Implementations producing Activity Streams 2.0 documents
/// should include a @context property with a value that includes a reference to
/// the normative Activity Streams 2.0 JSON-LD @context definition using the URL
/// "https://www.w3.org/ns/activitystreams". Implementations may use the
/// alternative URL "http://www.w3.org/ns/activitystreams" instead. This can be
/// done using a string, object, or array.
/// <https://www.w3.org/TR/activitystreams-core/#jsonld>

const NAMESPACE: &'static str = "https://www.w3.org/ns/activitystreams";

#[derive(Serialize, Deserialize, Debug, Clone, Builder)]
#[builder(default)]
pub struct Context {
    #[serde(rename = "@vocab")]
    namespace: String,

    #[serde(skip_serializing_if = "Option::is_none", rename = "@language")]
    language: Option<String>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            namespace: NAMESPACE.to_string(),
            language: None,
        }
    }
}
impl Default for Context {
    fn default() -> Self {
        Context {
            namespace: NAMESPACE.to_string(),
            language: None,
        }
    }
}

fn context_deserializer<'de, D>(deserializer: D) -> Result<Context, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize, Debug)]
    #[serde(untagged)]
    enum ContextType<'a> {
        Context(Context),
        Str(&'a str),
    }

    Ok(match ContextType::deserialize(deserializer)? {
        ContextType::Str(_x) => Context::new(),
        ContextType::Context(x) => x,
    })
}

impl ContextBuilder {
    pub fn new() -> Self {
        ContextBuilder::default()
    }
}

/// A [Collection] is a subtype of [Object] that represents ordered or unordered
/// sets of [Object] or [Link] instances. Refer to the Activity Streams 2.0 Core
/// specification for a complete description of the [Collection] type.
#[derive(Serialize, Deserialize, Debug, Clone, Builder)]
pub struct Collection<Item: Clone> { // TODO: can we avoid need for Clone?
    #[serde(flatten)]
    pub base: Object,

    #[serde(rename = "totalItems", skip_serializing_if = "Option::is_none")]
    pub total_items: Option<usize>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Item>,
}

impl<Item: Clone> CollectionBuilder<Item> {
    pub fn ordered_collection(items: Vec<Item>) -> Self
        where Item : DeserializeOwned + Serialize {
        let obj = ObjectBuilder::default()
            .object_type(Some("OrderedCollection".into()))
            .build()
            .unwrap();
        CollectionBuilder::default()
            .base(obj)
            .items(items)
            .to_owned()
    }
}

/// A subtype of [Collection] in which members of the logical collection are
/// assumed to always be strictly ordered.
#[derive(Serialize, Deserialize, Debug, Clone, Builder)]
pub struct OrderedCollection<CollectionT> {
    #[serde(flatten)]
    pub base: Object,

    #[serde(rename = "totalItems", skip_serializing_if = "Option::is_none")]
    pub total_items: Option<usize>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "orderedItems")]
    pub ordered_items: Vec<CollectionT>,
}

/// Used to represent distinct subsets of items from a [Collection]. Refer to
/// the Activity Streams 2.0 Core for a complete description of the
/// [CollectionPage] object.
#[derive(Serialize, Deserialize, Debug, Builder)]
pub struct CollectionPage<BaseCollection: Clone> {
    #[serde(flatten)]
    pub base: Collection<BaseCollection>,

    #[serde(rename = "partOf")]
    pub part_of: String,

    pub next: Option<String>,

    pub prev: Option<String>,
}

/// Used to represent ordered subsets of items from an [OrderedCollection].
/// Refer to the Activity Streams 2.0 Core for a complete description of
/// the [OrderedCollectionPage] object.
#[derive(Serialize, Deserialize, Debug, Builder)]
pub struct OrderedCollectionPage<BaseCollection: Clone> {
    #[serde(flatten)]
    pub base: OrderedCollection<BaseCollection>,

    #[serde(rename = "partOf")]
    pub part_of: String,

    pub next: Option<String>,

    pub prev: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_context() {
        let ctx: Context = ContextBuilder::default()
            .language(Some("en".into()))
            .build().unwrap();

        let expected = r#"{
  "@vocab": "https://www.w3.org/ns/activitystreams",
  "@language": "en"
}"#;
        let pretty_print = serde_json::to_string_pretty(&ctx);
        assert!(pretty_print.is_ok());
        assert_eq!(pretty_print.ok().unwrap(), expected)
    }

    #[test]
    fn deserialize_context() {
        let actual = String::from(r#"{
    "@vocab": "https://www.w3.org/ns/activitystreams",
    "@language": "en"
}"#,
        );
        let ctx: Context = serde_json::from_str(&actual).unwrap();
        assert_eq!(ctx.language, Some("en".into()));
        assert_eq!(ctx.namespace, "https://www.w3.org/ns/activitystreams".to_string());
}

}
