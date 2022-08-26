use crate::core::{ActivityStreamsObject, ActivityStreamsObjectBuilder, ActivityStreamsSerialize};
use http::Uri;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Actor {
    #[serde(flatten)]
    base: ActivityStreamsObject,

    #[serde(rename = "preferredUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
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

impl ActivityStreamsSerialize for Actor {
    fn from_json(json: String) -> Self {
        ActorBuilder::new(ActivityStreamsObjectBuilder::new().build()).build()
    }
}

pub struct ActorBuilder {
    base: ActivityStreamsObjectBuilder,

    preferred_username: Option<String>,
    summary: Option<String>,
    url: Option<Uri>,
    inbox: Option<String>,
    outbox: Option<String>,
    followers: Option<String>,
    following: Option<String>,
    liked: Option<String>,
}

impl ActorBuilder {
    pub fn new(base: ActivityStreamsObject) -> Self {
        ActorBuilder {
            base: ActivityStreamsObjectBuilder::new(actor_type)
                .id(id)
                .name(name),
            preferred_username: None,
            summary: None,
            url: None,
            inbox: None,
            outbox: None,
            followers: None,
            following: None,
            liked: None,
        }
    }

    pub fn preferred_username(mut self, username: String) -> Self {
        self.preferred_username = Some(username);
        self
    }

    pub fn summary(mut self, summary: String) -> Self {
        self.summary = Some(summary);
        self
    }

    pub fn url(mut self, url: Uri) -> Self {
        self.url = Some(url);
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
            summary: self.summary,
            url: match self.url {
                None => None,
                u => u.unwrap().to_string(),
            },
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
    use crate::core::{ActivityStreamsObjectBuilder, ActivityStreamsSerialize};
    use crate::extended::ActorBuilder;
    use http::Uri;

    #[test]
    fn create_actor_object() {
        let actual = ActorBuilder::new(
            ActivityStreamsObjectBuilder::new()
                .object_type("Person".to_string())
                .id("https://example.com/person/1234".parse::<Uri>().unwrap())
                .name("name".to_string())
                .build(),
        )
        .preferred_username("dma".to_string())
        .build();
        let expected = String::from(
            r#"{
  "@context": [
    "https://www.w3.org/ns/activitystreams"
  ],
  "type": "Person",
  "id": "https://example.com/person/1234",
  "name": "name",
  "preferredUsername": "dma"
}"#,
        );
        assert_eq!(actual.to_json_pretty(), expected)
    }
}
