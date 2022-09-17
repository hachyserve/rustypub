use crate::core::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Actor<'a> {
    #[serde(flatten, borrow)]
    base: Object<'a, Null>,

    #[serde(rename = "preferredUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_username: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbox: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub following: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liked: Option<&'a str>,
}

impl<'de: 'a, 'a> Serde<'de> for Actor<'a> {}

impl<'a> std::ops::Deref for Actor<'a> {
    type Target = Object<'a, Null>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

#[derive(Clone)]
pub struct ActorBuilder<'a> {
    base: ObjectBuilder<'a, Null>,

    preferred_username: Option<&'a str>,
    inbox: Option<&'a str>,
    outbox: Option<&'a str>,
    followers: Option<&'a str>,
    following: Option<&'a str>,
    liked: Option<&'a str>,
}

impl<'a> ActorBuilder<'a> {
    pub fn new(actor_type: &'a str) -> Self {
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

    pub fn name(mut self, name: &'a str) -> Self {
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

    pub fn image(mut self, image: LinkBuilder<'a>) -> Self {
        self.base.image(image);
        self
    }

    pub fn summary(mut self, summary: &'a str) -> Self {
        self.base.summary(summary);
        self
    }

    pub fn preferred_username(mut self, username: &'a str) -> Self {
        self.preferred_username = Some(username);
        self
    }

    pub fn inbox(mut self, inbox: &'a str) -> Self {
        self.inbox = Some(inbox);
        self
    }

    pub fn outbox(mut self, outbox: &'a str) -> Self {
        self.outbox = Some(outbox);
        self
    }

    pub fn followers(mut self, followers: &'a str) -> Self {
        self.followers = Some(followers);
        self
    }

    pub fn following(mut self, following: &'a str) -> Self {
        self.following = Some(following);
        self
    }

    pub fn liked(mut self, liked: &'a str) -> Self {
        self.liked = Some(liked);
        self
    }

    pub fn build(self) -> Actor<'a> {
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
            ActorBuilder::new("Person")
                .id("https://example.com/person/1234"
                    .parse::<http::Uri>()
                    .unwrap())
                .name("name")
                .preferred_username("dma")
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
        let document: Document<Actor> = Document::from_json(actual).unwrap();
        let actor = document.object as Actor;
        assert_eq!(actor.object_type, Some("Person"));
        assert_eq!(
            actor.id,
            Some(String::from("https://example.com/person/1234"))
        );
        assert_eq!(actor.name, Some("name"));
        assert_eq!(actor.preferred_username, Some("dma"));
    }
}
