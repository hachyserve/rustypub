use crate::core::ActivityStreamsObject;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Actor {
    #[serde(flatten)]
    root: ActivityStreamsObject,

    #[serde(rename = "type")]
    actor_type: String,
    #[serde(rename = "preferredUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,
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

impl Actor {
    pub fn to_json(&self) -> String {
        let serialized = serde_json::to_string(&self).unwrap();
        println!("serialized = {}", serialized);

        return serialized;
    }
}

pub struct ActorBuilder {
    root: ActivityStreamsObject,

    actor_type: String,
    preferred_username: Option<String>,
    summary: Option<String>,
    inbox: Option<String>,
    outbox: Option<String>,
    followers: Option<String>,
    following: Option<String>,
    liked: Option<String>,
}

impl ActorBuilder {
    pub fn new(id: String, name: String) -> Self {
        ActorBuilder {
            root: ActivityStreamsObject::new(id, name),
            actor_type: "Person".to_string(),
            preferred_username: None,
            summary: None,
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
            root: self.root,

            actor_type: self.actor_type,
            preferred_username: self.preferred_username,
            summary: self.summary,
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
    use crate::extended::ActorBuilder;

    #[test]
    fn create_actor_object() {
        let actual = ActorBuilder::new(
            "https://example.com/person/1234".to_string(),
            "name".to_string(),
        )
        .preferred_username("dma".to_string())
        .build();
        let expected = String::from(
            r#"{"@context":["https://www.w3.org/ns/activitystreams#Object",{"@language":"en"}],"id":"https://example.com/person/1234","name":"name","type":"Person","preferredUsername":"dma"}"#,
        );
        assert_eq!(actual.to_json(), expected)
    }
}
