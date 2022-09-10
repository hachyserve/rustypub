use crate::core::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Actor {
    #[serde(flatten)]
    base: Object<Null>,

    #[serde(rename = "preferredUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inbox: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outbox: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    followers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    following: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    liked: Option<String>,
}

impl Serde<'_> for Actor {}

impl Actor {
    pub fn objectType(&self) -> Option<&String> {
        self.base.objectType()
    }

    pub fn id(&self) -> Option<&String> {
        self.base.id()
    }

    pub fn name(&self) -> Option<&String> {
        self.base.name()
    }

    pub fn url(&self) -> Option<&String> {
        self.base.url()
    }

    pub fn published(&self) -> Option<DateTime<Utc>> {
        self.base.published()
    }

    pub fn image(&self) -> Option<&Link> {
        self.base.image()
    }

    pub fn attributedTo(&self) -> &Vec<Null> {
        self.base.attributedTo()
    }

    pub fn audience(&self) -> Option<&Object<Null>> {
        self.base.audience()
    }

    pub fn content(&self) -> Option<&String> {
        self.base.content()
    }

    pub fn summary(&self) -> Option<&String> {
        self.base.summary()
    }
}

#[derive(Clone)]
pub struct ActorBuilder {
    base: ObjectBuilder<Null>,

    preferred_username: Option<String>,
    inbox: Option<String>,
    outbox: Option<String>,
    followers: Option<String>,
    following: Option<String>,
    liked: Option<String>,
}

impl<'a> ActorBuilder {
    pub fn new(actor_type: String) -> Self {
        ActorBuilder {
            base: ObjectBuilder::new().object_type(actor_type),
            preferred_username: None,
            inbox: None,
            outbox: None,
            followers: None,
            following: None,
            liked: None,
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

    pub fn build(self) -> Actor {
        Actor {
            base: self.base.build(),

            preferred_username: self.preferred_username,
            inbox: self.inbox,
            outbox: self.outbox,
            followers: self.followers,
            following: self.following,
            liked: self.liked,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::*;
    use crate::extended::{Actor, ActorBuilder};
    use pretty_assertions::assert_eq;

    #[test]
    fn serialize_actor() {
        let actual = Document::new(
            ContextBuilder::new().build(),
            ActorBuilder::new("Person".to_string())
                .id("https://example.com/person/1234"
                    .parse::<http::Uri>()
                    .unwrap())
                .name("name".to_string())
                .preferred_username("dma".to_string())
                .build(),
        );
        let expected = String::from(
            r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "type": "Person",
  "id": "https://example.com/person/1234",
  "name": "name",
  "preferredUsername": "dma"
}"#,
        );
        assert!(actual.to_json_pretty().is_ok());
        assert_eq!(actual.to_json_pretty().unwrap(), expected)
    }

    #[test]
    fn deserialize_actor() {
        let actual = String::from(
            r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "type": "Person",
  "id": "https://example.com/person/1234",
  "name": "name",
  "preferredUsername": "dma"
}"#,
        );
        let document: Document<Actor> = Document::from_json(&actual).unwrap();
        let actor = document.object as Actor;
        assert_eq!(actor.objectType(), Some(&"Person".to_string()));
        assert_eq!(
            actor.id(),
            Some(&"https://example.com/person/1234".to_string())
        );
        assert_eq!(actor.name(), Some(&"name".to_string()));
        assert_eq!(actor.preferred_username, Some("dma".to_string()));
    }
}
