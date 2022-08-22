use serde::{Deserialize, Serialize};
use serde_tuple::*;

#[derive(Serialize_tuple, Deserialize_tuple, Debug)]
pub struct ActivityStreamContext {
    pub namespace: String,

    pub lang: ActivityStreamContextLanguage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityStreamContextLanguage {
    #[serde(rename = "@language")]
    pub language: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityStreamsObject {
    #[serde(rename = "@context")]
    pub context: ActivityStreamContext,
    pub id: String,
    pub name: String,
}

impl ActivityStreamsObject {
    pub const NAMESPACE: &'static str = "https://www.w3.org/ns/activitystreams";
    pub const TYPE: &'static str = "Object";
    pub fn new(id: String, name: String) -> Self {
        return ActivityStreamsObject {
            context: ActivityStreamContext {
                namespace: Self::NAMESPACE.to_string() + "#" + &*Self::TYPE.to_string(),
                lang: ActivityStreamContextLanguage {
                    language: "en".to_string(),
                },
            },
            id,
            name,
        };
    }

    pub fn json(&self) -> String {
        let serialized = serde_json::to_string(&self).unwrap();
        println!("serialized = {}", serialized);

        return serialized;
    }
}

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

    pub fn json(&self) -> String {
        let serialized = serde_json::to_string(&self).unwrap();
        println!("serialized = {}", serialized);

        return serialized;
    }
}

#[cfg(test)]
mod tests {
    use crate::{ActivityStreamsObject, Actor};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn create_activity_stream_object() {
        let actual = ActivityStreamsObject::new("id".to_string(), "name".to_string());
        let expected = String::from(
            r#"{"@context":["https://www.w3.org/ns/activitystreams#Object",{"@language":"en"}],"id":"id","name":"name"}"#,
        );
        assert_eq!(actual.json(), expected)
    }

    #[test]
    fn create_actor_object() {
        let actual = Actor::new(
            "https://example.com/person/1234".to_string(),
            "name".to_string(),
        );
        let expected = String::from(
            r#"{"@context":["https://www.w3.org/ns/activitystreams#Object",{"@language":"en"}],"id":"https://example.com/person/1234","name":"name","type":"Person"}"#,
        );
        assert_eq!(actual.json(), expected)
    }
}
