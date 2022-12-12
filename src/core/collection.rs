use serde::{ Deserialize, Serialize };
use derive_builder::Builder;
use super::object::{ Object, ObjectBuilder };

/// A [Collection] is a subtype of [Object] that represents ordered or unordered
/// sets of [Object] or [Link] instances. Refer to the Activity Streams 2.0 Core
/// specification for a complete description of the [Collection] type.
#[derive(Serialize, Deserialize, Debug, Clone, Builder)]
pub struct Collection<Item: Clone> { // TODO: can we avoid need for Clone?
    #[serde(flatten)]
    pub base: Object,

    #[serde(rename = "totalItems", skip_serializing_if = "Option::is_none")]
    pub total_items: Option<usize>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Item>,
}

impl<Item: Clone> CollectionBuilder<Item> {

    pub fn with_base<F>(&mut self, build_fn: F) -> &mut Self
        where F: FnOnce(&mut ObjectBuilder) -> &mut ObjectBuilder
    {
        let mut base_builder = ObjectBuilder::default();
        self.base(build_fn(&mut base_builder).build().unwrap())
    }

}

/// A subtype of [Collection] in which members of the logical collection are
/// assumed to always be strictly ordered.
#[derive(Serialize, Deserialize, Debug, Clone, Builder)]
pub struct OrderedCollection<CollectionT> {
    #[serde(flatten)]
    pub base: Object,

    #[serde(rename = "totalItems", skip_serializing_if = "Option::is_none")]
    pub total_items: Option<usize>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "orderedItems")]
    pub ordered_items: Vec<CollectionT>,
}

/// Used to represent distinct subsets of items from a [Collection]. Refer to
/// the Activity Streams 2.0 Core for a complete description of the
/// [CollectionPage] object.
#[derive(Serialize, Deserialize, Debug, Builder)]
pub struct CollectionPage<BaseCollection: Clone> {
    #[serde(flatten)]
    pub base: Collection<BaseCollection>,

    #[serde(rename = "partOf")]
    pub part_of: String,

    pub next: Option<String>,

    pub prev: Option<String>,
}

/// Used to represent ordered subsets of items from an [OrderedCollection].
/// Refer to the Activity Streams 2.0 Core for a complete description of
/// the [OrderedCollectionPage] object.
#[derive(Serialize, Deserialize, Debug, Builder)]
pub struct OrderedCollectionPage<BaseCollection: Clone> {
    #[serde(flatten)]
    pub base: OrderedCollection<BaseCollection>,

    #[serde(rename = "partOf")]
    pub part_of: String,

    pub next: Option<String>,

    pub prev: Option<String>,
}
