use std::fmt::Debug;

use erased_serde::serialize_trait_object;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{PhlowObject, PhlowView};

#[typetag::serialize(tag = "viewName")]
#[async_trait::async_trait]
pub trait PhlowViewSpecification: Send {
    async fn retrieve_items(&self) -> Vec<Box<dyn PhlowViewSpecificationListingItem>>;
    async fn retrieve_sent_item(&self, item: &PhlowObject) -> Option<PhlowObject>;
}

pub trait AsPhlowViewSpecification: PhlowView {
    fn create_view_specification(&self) -> Option<Box<dyn PhlowViewSpecification>>;
}

impl<V: PhlowView> AsPhlowViewSpecification for V {
    default fn create_view_specification(&self) -> Option<Box<dyn PhlowViewSpecification>> {
        None
    }
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PhlowViewSpecificationDataTransport {
    Included = 1,
    Lazy = 2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PhlowViewSpecificationListingType {
    Text,
}

pub trait PhlowViewSpecificationListingItem: erased_serde::Serialize + Send + Debug {
    fn phlow_object(&self) -> &PhlowObject;
}

serialize_trait_object!(PhlowViewSpecificationListingItem);

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PhlowViewSpecificationItemValue {
    #[serde(skip)]
    pub phlow_object: PhlowObject,
    pub item_text: String,
}

impl PhlowViewSpecificationListingItem for PhlowViewSpecificationItemValue {
    fn phlow_object(&self) -> &PhlowObject {
        &self.phlow_object
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PhlowViewSpecificationRowValue {
    #[serde(skip)]
    pub phlow_object: PhlowObject,
    pub column_values: Vec<PhlowViewSpecificationItemValue>,
}

impl PhlowViewSpecificationListingItem for PhlowViewSpecificationRowValue {
    fn phlow_object(&self) -> &PhlowObject {
        &self.phlow_object
    }
}
