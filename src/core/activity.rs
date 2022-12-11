use serde::{ Deserialize, Serialize };
use derive_builder::Builder;
use crate::core::object::{ Object, ObjectBuilder };
use crate::core::actor::Actor;

///////////////////////////////
// Activity
// ////////////////////////////
// One of the major sub-types of object
// Activity also has a number of subtypes itself

/// An [Activity] is a subtype of [Object] that describes some form of action
/// that may happen, is currently happening, or has already happened. The
/// [Activity] type itself serves as an abstract base type for all types of
/// activities. It is important to note that the [Activity] type itself does
/// not carry any specific semantics about the kind of action being taken.
#[derive(Serialize, Default, Deserialize, Debug, Clone, Builder)]
#[builder(default)]
pub struct Activity {
    #[serde(flatten)]
    pub base: Object,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<Actor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<Object>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<Object>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>, // TODO: Origin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument: Option<String>, // TODO: Instrument
}

impl ActivityBuilder {
    /// Instances of [IntransitiveActivity] are a subtype of [Activity] representing
    /// intransitive actions. The object property is therefore inappropriate for
    /// these activities.
    pub fn intransitive_activity(object_type: String) -> Self {
        let obj = ObjectBuilder::default()
            .object_type(Some(object_type))
            .build().unwrap();
        ActivityBuilder::default()
            .base(obj)
            .object(None).to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use crate::core::{ Document, ContextBuilder, object::ObjectBuilder, actor::ActorBuilder };

    #[test]
    fn serialize_activity() {
        let base_object = ObjectBuilder::default()
            .object_type(Some("Activity".into()))
            .summary(Some("Sally did something to a note".into())).build().unwrap();
        let target_object = ObjectBuilder::new()
                .object_type(Some("Note".into()))
                .name(Some("A Note".into()))
                .build().unwrap();
        let subject_actor = ActorBuilder::default()
            .base(
                ObjectBuilder::default()
                .object_type(Some("Person".into()))
                .name(Some("Sally".into()))
                .build().unwrap()
            ).build().unwrap();
        let activity = ActivityBuilder::default()
            .base(base_object)
            .object(Some(target_object))
            .actor(Some(subject_actor))
            .build().unwrap();
        let actual = Document::new(ContextBuilder::default().build().unwrap(), activity);
        let expected = String::from(
            r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "type": "Activity",
  "summary": "Sally did something to a note",
  "actor": {
    "type": "Person",
    "name": "Sally"
  },
  "object": {
    "type": "Note",
    "name": "A Note"
  }
}"#,
        );
        assert!(actual.pretty_print().is_ok());
        assert_eq!(actual.pretty_print().unwrap(), expected);
    }

    #[test]
    fn deserialize_activity() {
        let actual = String::from(
            r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "type": "Activity",
  "summary": "Sally did something to a note",
  "actor": {
    "type": "Person",
    "name": "Sally"
  },
  "object": {
    "type": "Note",
    "name": "A Note"
  }
}"#,
        );
        let document: Document<Activity> = Document::deserialize_string(actual).unwrap();
        let activity = document.object as Activity;
        assert_eq!(activity.base.object_type, Some("Activity".into()));
        assert_eq!(
            activity.base.summary,
            Some(String::from("Sally did something to a note"))
        );

        assert!(activity.actor.is_some());
        let actor = activity.actor.unwrap();
        assert_eq!(actor.base.object_type, Some(String::from("Person")));
        assert_eq!(actor.base.name, Some(String::from("Sally")));

        assert!(activity.object.is_some());
        let object = activity.object.as_ref().unwrap();
        assert_eq!(object.object_type, Some(String::from("Note")));
        assert_eq!(object.name, Some(String::from("A Note")));
    }
}
