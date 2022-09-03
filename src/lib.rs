mod core;
mod extended;

#[cfg(test)]
mod tests {
    use chrono::{DateTime, NaiveDate, Utc};
    use http::Uri;
    use pretty_assertions::assert_eq;

    use crate::{core::*, extended::ActorBuilder};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    // A set of tests from https://www.w3.org/TR/activitystreams-core/ examples
    #[test]
    fn minimal_activity_3_1() {
        let actual = ActivityStreamsDocument::new(
            ActivityStreamsContextBuilder::new().build(),
            ActivityStreamsActivityBuilder::new(
                "Create".to_string(),
                "Martin created an image".to_string(),
            )
            .actor(
                ActorBuilder::new("Person".to_string())
                    .id("http://www.test.example/martin".parse::<Uri>().unwrap()),
            )
            .object(
                ActivityStreamsObjectBuilder::new()
                    .id("http://example.org/foo.jpg".parse::<Uri>().unwrap()),
            )
            .build(),
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
        assert_eq!(actual.to_json_pretty(), expected);
    }

    #[test]
    fn basic_activity_with_additional_detail_3_2() {
        let actual = ActivityStreamsDocument::new(
            ActivityStreamsContextBuilder::new().build(),
            ActivityStreamsActivityBuilder::new(
                "Add".to_string(),
                "Martin added an article to his blog".to_string(),
            )
            // TODO: figure out how to get a 'Z' on this. probably requires a time-zone (so not naive)
            .published(DateTime::<Utc>::from_utc(
                NaiveDate::from_ymd(2015, 2, 10).and_hms(15, 4, 55),
                Utc,
            ))
            .actor(
                ActorBuilder::new("Person".to_string())
                    .id("http://www.test.example/martin".parse::<Uri>().unwrap())
                    .name("Martin Smith".to_string())
                    .url("http://example.org/martin".parse::<Uri>().unwrap())
                    .image(ActivityStreamsLinkBuilder::new(
                        ActivityStreamsUriBuilder::new(
                            "http://example.org/martin/image.jpg"
                                .parse::<Uri>()
                                .unwrap(),
                        )
                        .media_type("image/jpeg".to_string()),
                    )),
            )
            .object(
                ActivityStreamsObjectBuilder::new()
                    .object_type("Article".to_string())
                    .id("http://www.test.example/blog/abc123/xyz"
                        .parse::<Uri>()
                        .unwrap())
                    .name("Why I love Activity Streams".to_string())
                    .url(
                        "http://example.org/blog/2011/02/entry"
                            .parse::<Uri>()
                            .unwrap(),
                    ),
            )
            .target(
                ActivityStreamsObjectBuilder::new()
                    .object_type("OrderedCollection".to_string())
                    .id("http://example.org/blog/".parse::<Uri>().unwrap())
                    .name("Martin's Blog".to_string()),
            )
            .build(),
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
        assert_eq!(actual.to_json_pretty(), expected);
    }

    #[test]
    fn object_4_1_7() {
        let actual = ActivityStreamsDocument::new(
            ActivityStreamsContextBuilder::new().build(),
            ActivityStreamsObjectBuilder::new()
                .id("http://example.org/foo".parse::<Uri>().unwrap())
                .object_type("Note".to_string())
                .name("My favourite stew recipe".to_string())
                .published(DateTime::<Utc>::from_utc(
                    NaiveDate::from_ymd(2014, 8, 21).and_hms(12, 34, 56),
                    Utc,
                ))
                .add_attributed_to(
                    ActorBuilder::new("Person".to_string())
                        .id("http://joe.website.example/".parse::<Uri>().unwrap())
                        .name("Joe Smith".to_string())
                        .build(),
                )
                .build(),
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
        assert_eq!(actual.to_json_pretty(), expected);
    }
}
