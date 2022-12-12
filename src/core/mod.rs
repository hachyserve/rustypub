pub mod object;
pub mod activity;
pub mod actor;
pub mod collection;

pub use object::*;

use serde::{de::DeserializeOwned, Deserializer, Deserialize, Serialize};
use derive_builder::Builder;

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

    pub fn serialize_pretty(&self) -> serde_json::Result<String>  {
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
        let serialize_pretty = serde_json::to_string_pretty(&ctx);
        assert!(serialize_pretty.is_ok());
        assert_eq!(serialize_pretty.ok().unwrap(), expected)
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
