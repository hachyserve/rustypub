use crate::core::ActivityStreamsObject;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Actor {
    #[serde(flatten)]
    root: ActivityStreamsObject,

    #[serde(rename = "type")]
    pub actor_type: String,
    #[serde(rename = "preferredUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
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
}

impl Actor {
    pub fn new(id: String, name: String) -> Self {
        return Actor {
            root: ActivityStreamsObject::new(id, name),
            actor_type: "Person".to_string(),
            preferred_username: None,
            summary: None,
            inbox: None,
            outbox: None,
            followers: None,
            following: None,
            liked: None,
        };
    }

    pub fn to_json(&self) -> String {
        let serialized = serde_json::to_string(&self).unwrap();
        println!("serialized = {}", serialized);

        return serialized;
    }
}

#[cfg(test)]
mod tests {
    use crate::extended::Actor;

    #[test]
    fn create_actor_object() {
        let actual = Actor::new(
            "https://example.com/person/1234".to_string(),
            "name".to_string(),
        );
        let expected = String::from(
            r#"{"@context":["https://www.w3.org/ns/activitystreams#Object",{"@language":"en"}],"id":"https://example.com/person/1234","name":"name","type":"Person"}"#,
        );
        assert_eq!(actual.to_json(), expected)
    }
}
