use serde::{ Deserialize, Serialize };
use crate::core::object::{ Object, ObjectBuilder };
use derive_builder::Builder;

// TODO: expand to actor types: https://www.w3.org/TR/activitystreams-vocabulary/#actor-types
#[derive(Serialize, Deserialize, Default, Debug, Clone, Builder)]
#[builder(default)]
pub struct Actor {
    #[serde(flatten)]
    pub base: Object,

    #[serde(rename = "preferredUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_username: Option<String>,

    // TODO: spec says MUST have
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbox: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub following: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liked: Option<String>,

    #[serde(rename = "publicKey", skip_serializing_if = "Option::is_none")]
    pub public_key_info: Option<PublicKeyInfo>,
}

impl ActorBuilder {

    pub fn with_base<F>(&mut self, build_fn: F) -> &mut Self
        where F: FnOnce(&mut ObjectBuilder) -> &mut ObjectBuilder
    {
        let mut base_builder = ObjectBuilder::default();
        self.base(build_fn(&mut base_builder).build().unwrap())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyInfo {
    pub id: String,
    pub owner: String,
    pub public_key_pem: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{ContextBuilder, Document};
    use pretty_assertions::assert_eq;
    use http::Uri;

    #[test]
    fn serialize_actor() {
        let person = ActorBuilder::default()
            .with_base(|base|
                base.object_type(Some("Person".into()))
                .id(Some("https://example.com/person/1234".parse::<Uri>().unwrap()))
                .name(Some("name".into()))
            )
            .preferred_username(Some("dma".into()))
            .build().unwrap();
        let context = ContextBuilder::new().build().unwrap();
        let actual = Document::new(context, person);
        let expected = r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "type": "Person",
  "id": "https://example.com/person/1234",
  "name": "name",
  "preferredUsername": "dma"
}"#;
        let s = serde_json::to_string_pretty(&actual);
        assert!(s.is_ok());
        assert_eq!(s.unwrap(), expected)
    }

    #[test]
    fn deserialize_actor() {
        let actual = r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "type": "Person",
  "id": "https://example.com/person/1234",
  "name": "name",
  "preferredUsername": "dma"
}"#;
        let document: Document<Actor> = Document::deserialize_string(actual.into()).unwrap();
        let actor = document.object;
        assert_eq!(actor.base.object_type, Some("Person".into()));
        assert_eq!(actor.base.id, Some("https://example.com/person/1234".parse::<Uri>().unwrap()));
        assert_eq!(actor.base.name, Some("name".into()));
        assert_eq!(actor.preferred_username, Some("dma".into()));
    }
}
