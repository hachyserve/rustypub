use crate::core::actor::{Actor, ActorBuilder};
use crate::core::object::{Object, ObjectBuilder};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

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
    pub to: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>, // TODO: Origin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument: Option<String>, // TODO: Instrument
}

impl ActivityBuilder {
    // TODO: macro
    pub fn with_base<F>(&mut self, build_fn: F) -> &mut Self
    where
        F: FnOnce(&mut ObjectBuilder) -> &mut ObjectBuilder,
    {
        let mut base_builder = ObjectBuilder::default();
        self.base(build_fn(&mut base_builder).build().unwrap())
    }

    pub fn with_object<F>(&mut self, build_fn: F) -> &mut Self
    where
        F: FnOnce(&mut ObjectBuilder) -> &mut ObjectBuilder,
    {
        let mut base_builder = ObjectBuilder::default();
        self.object(Some(build_fn(&mut base_builder).build().unwrap()))
    }

    pub fn with_actor<F>(&mut self, build_fn: F) -> &mut Self
    where
        F: FnOnce(&mut ActorBuilder) -> &mut ActorBuilder,
    {
        let mut base_builder = ActorBuilder::default();
        self.actor(Some(build_fn(&mut base_builder).build().unwrap()))
    }

    pub fn with_target<F>(&mut self, build_fn: F) -> &mut Self
    where
        F: FnOnce(&mut ObjectBuilder) -> &mut ObjectBuilder,
    {
        let mut base_builder = ObjectBuilder::default();
        self.target(Some(build_fn(&mut base_builder).build().unwrap()))
    }
    /// Instances of [IntransitiveActivity] are a subtype of [Activity] representing
    /// intransitive actions. The object property is therefore inappropriate for
    /// these activities.
    pub fn intransitive_activity(object_type: String) -> Self {
        let obj = ObjectBuilder::default()
            .object_type(Some(object_type))
            .build()
            .unwrap();
        ActivityBuilder::default().base(obj).object(None).to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{ContextBuilder, Document};
    use pretty_assertions::assert_eq;

    #[test]
    fn serialize_activity() {
        let activity = ActivityBuilder::default()
            .with_base(|builder| {
                builder
                    .object_type(Some("Activity".into()))
                    .summary(Some("Sally did something to a note".into()))
            })
            .with_object(|builder| {
                builder
                    .object_type(Some("Note".into()))
                    .name(Some("A Note".into()))
            })
            .with_actor(|actor| {
                actor.with_base(|builder| {
                    builder
                        .object_type(Some("Person".into()))
                        .name(Some("Sally".into()))
                })
            })
            .build()
            .unwrap();
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
        assert!(actual.serialize_pretty().is_ok());
        assert_eq!(actual.serialize_pretty().unwrap(), expected);
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
            Some("Sally did something to a note".into())
        );

        assert!(activity.actor.is_some());
        let actor = activity.actor.unwrap();
        assert_eq!(actor.base.object_type, Some("Person".into()));
        assert_eq!(actor.base.name, Some("Sally".into()));

        assert!(activity.object.is_some());
        let object = activity.object.as_ref().unwrap();
        assert_eq!(object.object_type, Some("Note".into()));
        assert_eq!(object.name, Some("A Note".into()));
    }
}
