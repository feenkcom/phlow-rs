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

#[typetag::serialize(tag = "valueTypeName")]
pub trait PhlowViewSpecificationListingItem: Send + Debug {
    fn phlow_object(&self) -> &PhlowObject;
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PhlowViewSpecificationTextualItemValue {
    #[serde(skip)]
    pub phlow_object: PhlowObject,
    pub item_text: String,
}

#[typetag::serialize(name = "textualValue")]
impl PhlowViewSpecificationListingItem for PhlowViewSpecificationTextualItemValue {
    fn phlow_object(&self) -> &PhlowObject {
        &self.phlow_object
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PhlowViewSpecificationRowValue {
    #[serde(skip)]
    pub phlow_object: PhlowObject,
    pub column_values: Vec<Box<dyn PhlowViewSpecificationListingItem>>,
}

impl PhlowViewSpecificationRowValue {
    fn phlow_object(&self) -> &PhlowObject {
        &self.phlow_object
    }
}

#[typetag::serialize(name = "rowValue")]
impl PhlowViewSpecificationListingItem for PhlowViewSpecificationRowValue {
    fn phlow_object(&self) -> &PhlowObject {
        &self.phlow_object
    }
}
