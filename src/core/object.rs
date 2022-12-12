use serde::{ Deserialize, Serialize };
use chrono::{ DateTime, Utc };
use derive_builder::Builder;
use http::Uri;

///////////////////////////
// Object
///////////////////////////
/// The [Object] is the primary base type for the Activity Streams vocabulary.
/// In addition to having a global identifier (expressed as an absolute IRI
/// using the id property) and an "object type" (expressed using the type
/// property), all instances of the Object type share a common set of
/// properties normatively defined by the Activity Vocabulary. These
/// include: attachment | attributedTo | audience | content | context |
/// contentMap | name | nameMap | endTime | generator | icon | image |
/// inReplyTo | location | preview | published | replies | startTime |
/// summary | summaryMap | tag | updated | url | to | bto | cc | bcc |
/// mediaType | duration
/// All properties are optional (including the id and type).
#[derive(Serialize, Deserialize, Default, Debug, Clone, Builder)]
#[builder(default)]
pub struct Object {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    // TODO: actually an IRI: consider https://docs.rs/iref/latest/iref/
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Link>,

    #[serde(
        rename = "attributedTo",
        skip_serializing_if = "Vec::is_empty",
        default = "Vec::new",
    )]
    pub attributed_to: Vec<AttributedTo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<Box<Object>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<Box<Preview>>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Application(Object);

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Group(Object);

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Organization(Object);

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Person(Object);

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Service(Object);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)] 
pub enum AttributedTo {
    Object(Object),
    Link(Link)
}

impl ObjectBuilder {
    pub fn new() -> Self {
        ObjectBuilder::default()
    }

    pub fn of_object_type(t: String) -> Self {
        ObjectBuilder::default()
            .object_type(Some(t)).to_owned()
    }

    pub fn note(name: String, content: String) -> Self {
        ObjectBuilder::of_object_type("Note".into())
             .name(Some(name))
             .content(Some(content)).to_owned()
    }
}

///////////////////////////
// Link
///////////////////////////
/// A [Link] is an indirect, qualified reference to a resource identified by a
/// URL. The fundamental model for links is established by
/// [RFC5988](https://www.w3.org/TR/activitystreams-vocabulary/#bib-RFC5988).
/// Many of the properties defined by the Activity Vocabulary allow values that
/// are either instances of [Object] or [Link]. When a [Link] is used, it
/// establishes a qualified relation connecting the subject (the containing
/// object) to the resource identified by the href. Properties of the [Link]
/// are properties of the reference as opposed to properties of the resource.

#[derive(Serialize, Deserialize, Debug, Default, Clone, Builder)]
#[builder(default)]
pub struct Link {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub link_type: Option<String>,

    #[serde(with = "http_serde::uri")]
    pub href: Uri,

    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    pub rel: Vec<String>, // TODO: RFC5988 validation

    #[serde(rename = "mediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hreflang: Option<String>, // TODO: BCP47 language tag

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<Preview>,
}

impl Link {
    pub fn new(uri: String, media_type: String) -> Self {
        Link {
            link_type: Some("Link".into()),
            href: uri.parse().unwrap(),
            rel: vec![],
            media_type: Some(media_type),
            name: None,
            hreflang: None,
            height: None,
            width: None,
            preview: None
        }
    }
}

impl LinkBuilder {
    pub fn new() -> Self {
        LinkBuilder::default()
    }
}

///////////////////////////
// Preview
///////////////////////////
/// Identifies an entity that provides a preview of this object.
#[derive(Serialize, Deserialize, Debug, Clone, Builder)]
#[builder(default)]
pub struct Preview {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Box<Link>>,
}

impl Default for Preview {
    fn default() -> Self {
        Preview {
            object_type: Some("Preview".into()),
            name: None,
            duration: None,
            url: None,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::Result;
    use crate::core::{ Context, ContextBuilder, Document, DocumentBuilder };

    #[test]
    fn serialize_object() {
        let object: Object = ObjectBuilder::default()
            .name(Some("name".into()))
            .build().unwrap();
        let context: Context = ContextBuilder::new()
            .language(Some("en".into()))
            .build().unwrap();
        let actual = DocumentBuilder::default()
            .object(Some(object))
            .context(context)
            .build().unwrap();

        let expected = r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams",
    "@language": "en"
  },
  "name": "name"
}"#;
        let serialize_pretty = serde_json::to_string_pretty(&actual);
        assert!(serialize_pretty.is_ok());
        assert_eq!(serialize_pretty.ok().unwrap(), expected)
    }

    #[test]
    fn deserialize_object() {
        let actual = String::from(
            r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams",
    "@language": "en"
  },
  "name": "name"
}"#,
        );
        let document: Document<Object> = Document::deserialize_string(actual).unwrap();
        assert_eq!(document.context.language, Some("en".into()));
        let object = document.object as Object;
        assert_eq!(object.name, Some("name".into()));
    }

    #[test]
    fn deserialize_object_malformed() {
        let actual = String::from(
            r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams",
    "@language": "en"
  },
}"#,
        );
        let result: Result<Document<Object>> = Document::deserialize_string(actual);
        assert!(result.is_err());
    }

    #[test]
    fn serialize_link() {
        let href = Uri::from(
            "http://example.org/abc".parse::<http::Uri>().unwrap(),
        );
        let actual = Document::new(
            ContextBuilder::new().build().unwrap(),
            LinkBuilder::new()
            .href(href)
            .name(Some("An example link".into()))
            .hreflang(Some("en".into()))
            .link_type(Some("Link".into()))
            .media_type(Some("text/html".into()))
            .build().unwrap(),
        );
        let expected = String::from(
            r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "type": "Link",
  "href": "http://example.org/abc",
  "mediaType": "text/html",
  "name": "An example link",
  "hreflang": "en"
}"#
        );
        assert!(actual.serialize_pretty().is_ok());
        assert_eq!(actual.serialize_pretty().unwrap(), expected);
    }

    #[test]
    fn deserialize_link() {
        let actual = String::from(
            r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "type": "Link",
  "href": "http://example.org/abc",
  "name": "An example link",
  "hreflang": "en"
}"#,
        );
        let document: Document<Link> = Document::deserialize_string(actual).unwrap();
        let link = document.object as Link;
        assert_eq!(link.link_type, Some("Link".into()));
        assert_eq!(link.href, "http://example.org/abc");
        assert_eq!(link.name, Some("An example link".into()));
        assert_eq!(link.hreflang, Some("en".into()));
    }

    #[test]
    fn serialize_preview() {
         let trailer_preview = Link::new(
            "http://example.org/trailer.mkv".into(),
            "video/mkv".into()
        );
        let preview = PreviewBuilder::default()
            .duration(Some("PT1M".into()))
            .object_type(Some("Video".into()))
            .url(Some(Box::new(trailer_preview)))
            .name(Some("Trailer".into()))
            .build().unwrap();

        let object = ObjectBuilder::default()
            .duration(Some("PT2H30M".into()))
            .name(Some("Cool New Movie".into()))
            .preview(Some(Box::new(preview)))
            .object_type(Some("Video".into()))
            .build().unwrap();
        let context = ContextBuilder::new()
            .build().unwrap();
        let actual = Document::new(context, object);
        let expected = String::from(
            r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "type": "Video",
  "name": "Cool New Movie",
  "duration": "PT2H30M",
  "preview": {
    "type": "Video",
    "name": "Trailer",
    "duration": "PT1M",
    "url": {
      "type": "Link",
      "href": "http://example.org/trailer.mkv",
      "mediaType": "video/mkv"
    }
  }
}"#,
        );
        assert!(actual.serialize_pretty().is_ok());
        assert_eq!(actual.serialize_pretty().unwrap(), expected);
    }

    #[test]
    fn deserialize_preview() {
        let actual = String::from(
            r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "type": "Video",
  "name": "Cool New Movie",
  "duration": "PT2H30M",
  "preview": {
    "type": "Video",
    "name": "Trailer",
    "duration": "PT1M",
    "url": {
      "href": "http://example.org/trailer.mkv",
      "mediaType": "video/mkv"
    }
  }
}"#,
        );
        let document: Document<Object> = Document::deserialize_string(actual).unwrap();
        let object = document.object;
        assert_eq!(object.object_type, Some("Video".into()));
        assert_eq!(object.name, Some("Cool New Movie".into()));
        assert_eq!(object.duration, Some("PT2H30M".into()));

        let preview = object.preview.unwrap();
        assert_eq!(preview.object_type, Some("Video".into()));
        assert_eq!(preview.name, Some("Trailer".into()));
        assert_eq!(preview.duration, Some("PT1M".into()));

        let url = preview.url.as_ref().unwrap();
        assert_eq!(url.media_type, Some("video/mkv".into()));
        assert_eq!(url.href, "http://example.org/trailer.mkv".parse::<http::Uri>().unwrap());
    }

    #[test]
    fn serialize_note() {
        let context = ContextBuilder::new().build().unwrap();
        let note = ObjectBuilder::note("Name".into(), "Content".into()).build().unwrap();
        let document: Document<Object> = Document::new(context, note);
        let expected = r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "type": "Note",
  "name": "Name",
  "content": "Content"
}"#;
        assert!(document.serialize_pretty().is_ok());
        assert_eq!(document.serialize_pretty().unwrap(), expected)
    }

    #[test]
    fn deserialize_note() {
        let actual = r#"{
  "@context": {
    "@vocab": "https://www.w3.org/ns/activitystreams"
  },
  "type": "Note",
  "name": "Name",
  "content": "Content"
}"#.into();
        let document: Document<Object> = Document::deserialize_string(actual).unwrap();
        let note: Object = document.object;
        assert_eq!(note.object_type, Some("Note".into()));
        assert_eq!(note.name, Some("Name".into()));
        assert_eq!(note.content, Some("Content".into()));
    }
}
