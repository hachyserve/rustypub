use crate::core::{LinkBuilder, Null, Object, ObjectBuilder};
use crate::Serde;
use chrono::{DateTime, Utc};
use rsa::{
    pkcs1::{DecodeRsaPublicKey, Error},
    RsaPublicKey,
};
use serde::{Deserialize, Serialize};

/// https://www.w3.org/TR/activitystreams-vocabulary/#dfn-note
#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    #[serde(flatten)]
    base: Object<Null>,
}

impl Note {
    pub fn new(name: String, content: String) -> Self {
        Note {
            base: ObjectBuilder::new()
                .object_type("Note".to_string())
                .name(name)
                .content(content)
                .build(),
        }
    }
}

impl Serde for Note {}

impl std::ops::Deref for Note {
    type Target = Object<Null>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

// TODO: expand to actor types: https://www.w3.org/TR/activitystreams-vocabulary/#actor-types
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Actor {
    #[serde(flatten)]
    base: Object<Null>,

    #[serde(rename = "preferredUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_username: Option<String>,
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

impl Serde for Actor {}

impl std::ops::Deref for Actor {
    type Target = Object<Null>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl Actor {
    pub fn key(&self) -> Result<RsaPublicKey, Error> {
        RsaPublicKey::from_pkcs1_pem(
            &self
                .public_key_info
                .as_ref()
                .expect("public_key_info not set")
                .public_key_pem,
        )
    }
}

/// Builder for an [Actor].
#[derive(Clone)]
pub struct ActorBuilder {
    base: ObjectBuilder<Null>,

    preferred_username: Option<String>,
    inbox: Option<String>,
    outbox: Option<String>,
    followers: Option<String>,
    following: Option<String>,
    liked: Option<String>,
    public_key_info: Option<PublicKeyInfo>,
}

impl ActorBuilder {
    pub fn new(actor_type: String) -> Self {
        ActorBuilder {
            base: ObjectBuilder::new().object_type(actor_type),
            preferred_username: None,
            inbox: None,
            outbox: None,
            followers: None,
            following: None,
            liked: None,
            public_key_info: None,
        }
    }

    pub fn id(mut self, id: http::Uri) -> Self {
        self.base.id(id);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.base.name(name);
        self
    }

    pub fn url(mut self, url: http::Uri) -> Self {
        self.base.url(url);
        self
    }

    pub fn published(mut self, datetime: DateTime<Utc>) -> Self {
        self.base.published(datetime);
        self
    }

    pub fn image(mut self, image: LinkBuilder) -> Self {
        self.base.image(image);
        self
    }

    pub fn summary(mut self, summary: String) -> Self {
        self.base.summary(summary);
        self
    }

    pub fn preferred_username(mut self, username: String) -> Self {
        self.preferred_username = Some(username);
        self
    }

    pub fn inbox(mut self, inbox: String) -> Self {
        self.inbox = Some(inbox);
        self
    }

    pub fn outbox(mut self, outbox: String) -> Self {
        self.outbox = Some(outbox);
        self
    }

    pub fn followers(mut self, followers: String) -> Self {
        self.followers = Some(followers);
        self
    }

    pub fn following(mut self, following: String) -> Self {
        self.following = Some(following);
        self
    }

    pub fn liked(mut self, liked: String) -> Self {
        self.liked = Some(liked);
        self
    }

    pub fn public_key_info(mut self, public_key_info: PublicKeyInfo) -> Self {
        self.public_key_info = Some(public_key_info.clone());
        self
    }

    pub fn build(self) -> Actor {
        Actor {
            base: self.base.build(),

            preferred_username: self.preferred_username,
            inbox: self.inbox,
            outbox: self.outbox,
            followers: self.followers,
            following: self.following,
            liked: self.liked,
            public_key_info: self.public_key_info,
        }
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

    #[test]
    fn serialize_actor() {
        let actual = Document::new(
            ContextBuilder::new().build(),
            ActorBuilder::new(String::from("Person"))
                .id("https://example.com/person/1234"
                    .parse::<http::Uri>()
                    .unwrap())
                .name(String::from("name"))
                .preferred_username(String::from("dma"))
                .build(),
        );
        let expected = r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "type": "Person",
  "id": "https://example.com/person/1234",
  "name": "name",
  "preferredUsername": "dma"
}"#;
        assert!(actual.to_json_pretty().is_ok());
        assert_eq!(actual.to_json_pretty().unwrap(), expected)
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
        let document: Document<Actor> = Document::from_json(String::from(actual)).unwrap();
        let actor = document.object as Actor;
        assert_eq!(actor.object_type, Some(String::from("Person")));
        assert_eq!(
            actor.id,
            Some(String::from("https://example.com/person/1234"))
        );
        assert_eq!(actor.name, Some(String::from("name")));
        assert_eq!(actor.preferred_username, Some(String::from("dma")));
    }

    #[test]
    fn serialize_note() {
        let actual = Document::new(
            ContextBuilder::new().build(),
            Note::new(String::from("Name"), String::from("Content")),
        );
        let expected = r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "type": "Note",
  "name": "Name",
  "content": "Content"
}"#;
        assert!(actual.to_json_pretty().is_ok());
        assert_eq!(actual.to_json_pretty().unwrap(), expected)
    }

    #[test]
    fn deserialize_note() {
        let actual = r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "type": "Note",
  "name": "Name",
  "content": "Content"
}"#;
        let document: Document<Note> = Document::from_json(String::from(actual)).unwrap();
        let actor = document.object as Note;
        assert_eq!(actor.object_type, Some(String::from("Note")));
        assert_eq!(actor.name, Some(String::from("Name")));
        assert_eq!(actor.content, Some(String::from("Content")));
    }
}
