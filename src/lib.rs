mod core;
mod extended;

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use http::Uri;

    use crate::{
        core::{
            ActivityStreamsActivityBuilder, ActivityStreamsObjectBuilder, ActivityStreamsSerialize,
        },
        extended::ActorBuilder,
    };

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    // A set of tests from https://www.w3.org/TR/activitystreams-core/ examples
    #[test]
    fn minimal_activity_3_1() {
        // TODO
        let actual = ActivityStreamsActivityBuilder::new(
            "Create".to_string(),
            "Martin created an image".to_string(),
        )
        .build();
        let expected = r#"{
  "@context": "https://www.w3.org/ns/activitystreams",
  "type": "Create",
  "summary": "Martin created an image",
  "actor": "http://www.test.example/martin",
  "object":"http://example.org/foo.jpg"
}"#;
        assert_eq!(actual.to_json_pretty(), expected);
    }

    #[test]
    fn basic_activity_with_additional_detail_3_2() {
        let actual = ActivityStreamsActivityBuilder::new(
            "Add".to_string(),
            "Martin created an image".to_string(),
        )
        .actor(
            ActorBuilder::new("Person".to_string())
                .id("http://www.test.example/martin".parse::<Uri>().unwrap())
                .name("Martin Smith".to_string())
                .url("http://example.org/martin".parse::<Uri>().unwrap())
                .published(NaiveDate::from_ymd(2015, 2, 10).and_hms(15, 4, 55)),
        ) // TODO: take a date-time and convert to string
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
        .build();
        let expected = r#"{
  "@context": "https://www.w3.org/ns/activitystreams",
  "summary": "Martin added an article to his blog",
  "type": "Add",
  "published": "2015-02-10T15:04:55Z",
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
  "object" : {
    "id": "http://www.test.example/blog/abc123/xyz",
    "type": "Article",
    "url": "http://example.org/blog/2011/02/entry",
    "name": "Why I love Activity Streams"
  },
  "target" : {
    "id": "http://example.org/blog/",
    "type": "OrderedCollection",
    "name": "Martin's Blog"
  }
}"#;
        assert_eq!(actual.to_json_pretty(), expected);
    }
}
