pub mod core;

extern crate serde;
extern crate derive_builder;


#[cfg(test)]
mod tests {
    use chrono::{DateTime, NaiveDate, Utc};
    use pretty_assertions::assert_eq;
    use http::Uri;

    use crate::core::{
        Document,
        ContextBuilder,
        collection::{ Collection, OrderedCollection, CollectionPage, OrderedCollectionPage },
        activity::{ Activity, ActivityBuilder },
        object::{ Object, ObjectBuilder, AttributedTo },
        Link,
    };

    // A set of tests from https://www.w3.org/TR/activitystreams-vocabulary examples
    #[test]
    fn example_1() {
        let listing = r#"{
          "@context": { "@vocab": "https://www.w3.org/ns/activitystreams" },
          "type": "Object",
          "id": "http://www.test.example/object/1",
          "name": "A Simple, non-specific object"
        }"#;
        let object: Object = Document::deserialize_string(listing.into()).unwrap().object;
        assert_eq!(object.object_type, Some("Object".into()));
        assert_eq!(
            object.id,
            Some("http://www.test.example/object/1".parse::<Uri>().unwrap())
        );
        assert_eq!(
            object.name,
            Some("A Simple, non-specific object".into())
        );
    }

    #[test]
    fn example_2() {
        let listing = r#"
      {
        "@context": {"@vocab": "https://www.w3.org/ns/activitystreams"},
        "type": "Link",
        "href": "http://example.org/abc",
        "hreflang": "en",
        "mediaType": "text/html",
        "name": "An example link"
      }
      "#;

        let link: Link = Document::deserialize_string(listing.into()).unwrap().object;
        assert_eq!(link.link_type, Some("Link".into()));
        assert_eq!(link.href, "http://example.org/abc".parse::<http::Uri>().unwrap());
        assert_eq!(link.hreflang, Some("en".into()));
        assert_eq!(link.media_type, Some("text/html".into()));
        assert_eq!(link.name, Some("An example link".into()));
    }

    #[test]
    fn example_3() {
        let listing = r#"
      {
        "@context": { "@vocab": "https://www.w3.org/ns/activitystreams" },
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
      } 
      "#;

        let activity: Activity = Document::deserialize_string(listing.into()).unwrap().object;
        assert_eq!(activity.base.object_type, Some("Activity".into()));
        assert_eq!( activity.base.summary, Some("Sally did something to a note".into()));

        assert!(activity.actor.is_some());
        let actor = activity.actor.unwrap();
        assert_eq!(actor.base.object_type, Some("Person".into()));
        assert_eq!(actor.base.name, Some("Sally".into()));

        assert!(activity.object.is_some());
        let object = activity.object.unwrap();
        assert_eq!(object.object_type, Some("Note".into()));
        assert_eq!(object.name, Some("A Note".into()));
    }

    #[test]
    fn example_4() {
        let listing = r#"
      {
        "@context": { "@vocab": "https://www.w3.org/ns/activitystreams" },
        "type": "Travel",
        "summary": "Sally went to work",
        "actor": {
          "type": "Person",
          "name": "Sally"
        },
        "target": {
          "type": "Place",
          "name": "Work"
        }
      }
      "#;

        let activity: Activity = Document::deserialize_string(listing.into()).unwrap().object;
        assert_eq!(activity.base.object_type, Some("Travel".into()));
        assert_eq!(activity.base.summary, Some("Sally went to work".into()));
        assert!(activity.object.is_none());

        assert!(activity.actor.is_some());
        let actor = activity.actor.as_ref().unwrap();
        assert_eq!(actor.base.object_type, Some("Person".into()));
        assert_eq!(actor.base.name, Some("Sally".into()));

        assert!(activity.target.is_some());
        let target = activity.target.as_ref().unwrap();
        assert_eq!(target.object_type, Some("Place".into()));
        assert_eq!(target.name, Some("Work".into()));
    }

    #[test]
    fn example_5() {
        let listing = r#"
        {
          "@context": { "@vocab": "https://www.w3.org/ns/activitystreams" },
          "summary": "Sally's notes",
          "type": "Collection",
          "totalItems": 2,
          "items": [
            {
              "type": "Note",
              "name": "A Simple Note"
            },
            {
              "type": "Note",
              "name": "Another Simple Note"
            }
          ]
        }
      "#;

        let collection: Collection<Object> = Document::deserialize_string(listing.into()).unwrap().object;
        assert_eq!(collection.base.object_type, Some("Collection".into()));
        assert_eq!(collection.base.summary, Some("Sally's notes".into()));
        assert_eq!(collection.total_items, Some(2));

        let items = &collection.items;
        assert_eq!(items.len(), collection.total_items.unwrap());
        assert_eq!(items[0].object_type, Some("Note".into()));
        assert_eq!(items[0].name, Some("A Simple Note".into()));
        assert_eq!(items[1].object_type, Some("Note".into()));
        assert_eq!(items[1].name, Some("Another Simple Note".into()));
    }

    #[test]
    fn example_6() {
        let listing = r#"
      {
        "@context": { "@vocab": "https://www.w3.org/ns/activitystreams" },
        "summary": "Sally's notes",
        "type": "OrderedCollection",
        "totalItems": 2,
        "orderedItems": [
          {
            "type": "Note",
            "name": "A Simple Note"
          },
          {
            "type": "Note",
            "name": "Another Simple Note"
          }
        ]
      }
      "#;
        let collection: OrderedCollection<Object> = Document::deserialize_string(listing.into()).unwrap().object;
        assert_eq!(
            collection.base.object_type,
            Some("OrderedCollection".into())
        );
        assert_eq!(collection.base.summary, Some("Sally's notes".into()));
        assert_eq!(collection.total_items, Some(2));

        let items = &collection.ordered_items;
        assert_eq!(items.len(), collection.total_items.unwrap());
        assert_eq!(items[0].object_type, Some("Note".into()));
        assert_eq!(items[0].name, Some("A Simple Note".into()));
        assert_eq!(items[1].object_type, Some("Note".into()));
        assert_eq!(items[1].name, Some("Another Simple Note".into()));
    }

    #[test]
    fn example_7() {
        let listing = r#"
      {
        "@context": { "@vocab": "https://www.w3.org/ns/activitystreams" },
        "summary": "Page 1 of Sally's notes",
        "type": "CollectionPage",
        "id": "http://example.org/foo?page=1",
        "partOf": "http://example.org/foo",
        "items": [
          {
            "type": "Note",
            "name": "A Simple Note"
          },
          {
            "type": "Note",
            "name": "Another Simple Note"
          }
        ]
      }
      "#;
        let collection_page: CollectionPage<Object> =
            Document::deserialize_string(listing.into()).unwrap().object;
        assert_eq!(
            collection_page.base.base.object_type,
            Some("CollectionPage".into())
        );
        assert_eq!(
            collection_page.base.base.id,
            Some("http://example.org/foo?page=1".parse::<Uri>().unwrap())
        );
        assert_eq!(
            collection_page.base.base.summary,
            Some("Page 1 of Sally's notes".into())
        );
        assert_eq!(
            collection_page.part_of,
            "http://example.org/foo".to_string()
        );
        assert_eq!(collection_page.base.total_items, None);

        let items = &collection_page.base.items;
        assert_eq!(items[0].object_type, Some("Note".into()));
        assert_eq!(items[0].name, Some("A Simple Note".into()));
        assert_eq!(items[1].object_type, Some("Note".into()));
        assert_eq!(items[1].name, Some("Another Simple Note".into()));
    }

    #[test]
    fn example_8() {
        let listing = r#"
{
  "@context": {"@vocab": "https://www.w3.org/ns/activitystreams"},
  "summary": "Page 1 of Sally's notes",
  "type": "OrderedCollectionPage",
  "id": "http://example.org/foo?page=1",
  "partOf": "http://example.org/foo",
  "orderedItems": [
    {
      "type": "Note",
      "name": "A Simple Note"
    },
    {
      "type": "Note",
      "name": "Another Simple Note"
    }
  ]
}
"#;
        let collection_page: OrderedCollectionPage<Object> =
            Document::deserialize_string(listing.into()).unwrap().object;
        assert_eq!(
            collection_page.base.base.object_type,
            Some("OrderedCollectionPage".into())
        );
        assert_eq!(
            collection_page.base.base.id,
            Some("http://example.org/foo?page=1".parse::<Uri>().unwrap())
        );
        assert_eq!(
            collection_page.base.base.summary,
            Some("Page 1 of Sally's notes".into())
        );
        assert_eq!(
            collection_page.part_of,
            "http://example.org/foo".to_string()
        );
        assert_eq!(collection_page.base.total_items, None);

        let items = &collection_page.base.ordered_items;
        assert_eq!(items[0].object_type, Some("Note".into()));
        assert_eq!(items[0].name, Some("A Simple Note".into()));
        assert_eq!(items[1].object_type, Some("Note".into()));
        assert_eq!(items[1].name, Some("Another Simple Note".into()));
    }

    #[test]
    fn example_53() {
        let listing = r#"{
          "@context": { "@vocab": "https://www.w3.org/ns/activitystreams" },
          "type": "Note",
          "name": "A Word of Warning",
          "content": "Looks like it is going to rain today. Bring an umbrella!"
        }"#;
        let document: Document<Object> = Document::deserialize_string(listing.into()).unwrap();
        let note = document.object;
        assert_eq!(note.object_type, Some("Note".into()));
        assert_eq!(note.name, Some("A Word of Warning".into()));
        assert_eq!(
            note.content,
            Some(String::from(
                "Looks like it is going to rain today. Bring an umbrella!"
            ))
        );
    }

    #[test]
    fn example_69() {
        let listing = r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "name": "Holiday announcement",
  "type": "Note",
  "content": "Thursday will be a company-wide holiday. Enjoy your day off!",
  "audience": {
    "type": "http://example.org/Organization",
    "name": "ExampleCo LLC"
  }
}"#;
        let document: Document<Object> = Document::deserialize_string(listing.into()).unwrap();
        let object = document.object;
        assert_eq!(object.name, Some("Holiday announcement".into()));
        assert_eq!(object.object_type, Some("Note".into()));
        assert_eq!(
            object.content,
            Some(String::from(
                "Thursday will be a company-wide holiday. Enjoy your day off!"
            ))
        );
        assert!(object.audience.is_some());
        let audience = object.audience.unwrap();
        assert_eq!(
            audience.object_type,
            Some("http://example.org/Organization".into())
        );
        assert_eq!(audience.name, Some("ExampleCo LLC".into()));
    }

    #[test]
    fn example_114() {
        let listing = r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "summary": "A simple note",
  "type": "Note",
  "content": "A <em>simple</em> note"
}"#;
        let document: Document<Object> = Document::deserialize_string(listing.into()).unwrap();
        let object = document.object;
        assert_eq!(object.summary, Some("A simple note".into()));
        assert_eq!(object.object_type, Some("Note".into()));
        assert_eq!(object.content, Some("A <em>simple</em> note".into()));
    }

    #[test]
    fn example_133() {
        let listing = r#"{
        "@context": {
          "@vocab": "https://www.w3.org/ns/activitystreams"
        },
        "name": "Cane Sugar Processing",
        "type": "Note",
        "summary": "A simple <em>note</em>"
      }"#;
        let document: Document<Object> = Document::deserialize_string(listing.into()).unwrap();
        let object = document.object;
        assert_eq!(object.summary, Some("A simple <em>note</em>".into()));
        assert_eq!(object.object_type, Some("Note".into()));
        assert_eq!(object.name, Some("Cane Sugar Processing".into()));
    }

    // A set of tests from https://www.w3.org/TR/activitystreams-core/ examples
    #[test]
    fn minimal_activity_3_1() {
        let activity = ActivityBuilder::default()
            .with_base(|builder|
                  builder.object_type(Some("Create".into()))
                  .summary(Some("Martin created an image".into()))
            )
            .with_actor(|actor|
                actor.with_base(|base_builder|
                    base_builder.object_type(Some("Person".into()))
                    .id(Some("http://www.test.example/martin".parse::<Uri>().unwrap()))
                )
            )
            .with_object(|builder| builder.id(Some("http://example.org/foo.jpg".parse::<Uri>().unwrap())))
            .build().unwrap();
        let actual = Document::new(
            ContextBuilder::new().build().unwrap(),
            activity
        );
        let expected = r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "type": "Create",
  "summary": "Martin created an image",
  "actor": {
    "type": "Person",
    "id": "http://www.test.example/martin"
  },
  "object": {
    "id": "http://example.org/foo.jpg"
  }
}"#;
        assert!(actual.serialize_pretty().is_ok());
        assert_eq!(actual.serialize_pretty().unwrap(), expected);
    }

    #[test]
    fn basic_activity_with_additional_detail_3_2() {
        let activity = ActivityBuilder::default()
            .with_base(|b|
                b.object_type(Some("Add".into()))
                .summary(Some("Martin added an article to his blog".into()))
                .published(Some(DateTime::<Utc>::from_utc(
                    NaiveDate::from_ymd(2015, 2, 10).and_hms(15, 4, 55),
                    Utc,
                )))
            )
            .with_actor(|actor|
                actor.with_base(|base_builder|
                    base_builder.object_type(Some("Person".into()))
                    .id(Some("http://www.test.example/martin".parse::<Uri>().unwrap()))
                    .name(Some("Martin Smith".into()))
                    .image(Some(Link::new(
                        "http://example.org/martin/image.jpg".into(),
                        "image/jpeg".into(),
                    )))
                    .url(Some("http://example.org/martin".into()))
                )
            )
            // TODO: figure out how to get a 'Z' on this. probably requires a time-zone (so not naive)
            .with_object(|builder|
                 builder.object_type(Some("Article".into()))
                 .id(Some("http://www.test.example/blog/abc123/xyz".parse::<Uri>().unwrap()))
                 .name(Some("Why I love Activity Streams".into()))
                 .url(Some("http://example.org/blog/2011/02/entry".into()))
            )
            .with_target(|target|
                target.object_type(Some("OrderedCollection".into()))
                .id(Some("http://example.org/blog/".parse::<Uri>().unwrap()))
                .name(Some("Martin's Blog".into()))
            )
            .build().unwrap();

        let actual = Document::new(
            ContextBuilder::new().build().unwrap(),
            activity
        );
        let expected = r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "type": "Add",
  "published": "2015-02-10T15:04:55Z",
  "summary": "Martin added an article to his blog",
  "actor": {
    "type": "Person",
    "id": "http://www.test.example/martin",
    "name": "Martin Smith",
    "url": "http://example.org/martin",
    "image": {
      "type": "Link",
      "href": "http://example.org/martin/image.jpg",
      "mediaType": "image/jpeg"
    }
  },
  "object": {
    "type": "Article",
    "id": "http://www.test.example/blog/abc123/xyz",
    "name": "Why I love Activity Streams",
    "url": "http://example.org/blog/2011/02/entry"
  },
  "target": {
    "type": "OrderedCollection",
    "id": "http://example.org/blog/",
    "name": "Martin's Blog"
  }
}"#;
        assert!(actual.serialize_pretty().is_ok());
        assert_eq!(actual.serialize_pretty().unwrap(), expected);
    }

    #[test]
    fn object_4_1_7() {
        let subject = ObjectBuilder::default()
            .object_type(Some("Person".into()))
            .id(Some("http://joe.website.example/".parse::<Uri>().unwrap()))
            .name(Some("Joe Smith".into()))
            .build().unwrap();
        let actual = Document::new(
            ContextBuilder::new().build().unwrap(),
            ObjectBuilder::new()
                .id(Some("http://example.org/foo".parse::<Uri>().unwrap()))
                .object_type(Some("Note".into()))
                .name(Some("My favourite stew recipe".into()))
                .published(Some(DateTime::<Utc>::from_utc(
                    NaiveDate::from_ymd(2014, 8, 21).and_hms(12, 34, 56),
                    Utc,
                )))
                .attributed_to(vec![AttributedTo::Object(subject)])
                .build().unwrap()
        );

        let expected = r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "type": "Note",
  "id": "http://example.org/foo",
  "name": "My favourite stew recipe",
  "published": "2014-08-21T12:34:56Z",
  "attributedTo": [
    {
      "type": "Person",
      "id": "http://joe.website.example/",
      "name": "Joe Smith"
    }
  ]
}"#;
        assert!(actual.serialize_pretty().is_ok());
        assert_eq!(actual.serialize_pretty().unwrap(), expected);
    }
}
