use std::any::Any;
use std::fmt::{Debug, Display, Formatter};

use crate::{PhlowObject, PhlowView, PhlowViewMethod, TypedPhlowObject, TypedPhlowObjectMut};

#[allow(unused)]
pub struct PhlowTextView {
    object: PhlowObject,
    defining_method: PhlowViewMethod,
    title: String,
    priority: usize,
    text_computation: Box<dyn Fn(&PhlowObject) -> String>,
}

impl PhlowTextView {
    pub fn new(object: PhlowObject, defining_method: PhlowViewMethod) -> Self {
        Self {
            object,
            defining_method,
            title: "".to_string(),
            priority: 10,
            text_computation: Box::new(|object| object.to_string()),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn priority(mut self, priority: usize) -> Self {
        self.priority = priority;
        self
    }

    pub fn text<T: 'static>(
        mut self,
        text_block: impl Fn(TypedPhlowObject<T>) -> String + 'static,
    ) -> Self {
        self.text_computation = Box::new(move |each_object| match each_object.value_ref::<T>() {
            Some(each_reference) => text_block(TypedPhlowObject::new(each_object, &each_reference)),
            None => "Error coercing item type".to_string(),
        });
        self
    }

    pub fn text_mut<T: 'static>(
        mut self,
        text_block: impl Fn(TypedPhlowObjectMut<T>) -> String + 'static,
    ) -> Self {
        self.text_computation = Box::new(move |each_object| match each_object.value_mut::<T>() {
            Some(mut each_reference) => {
                text_block(TypedPhlowObjectMut::new(each_object, &mut each_reference))
            }
            None => "Error coercing item type".to_string(),
        });
        self
    }

    pub fn compute_text(&self) -> String {
        (self.text_computation)(&self.object)
    }
}

impl Debug for PhlowTextView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PhlowTextView").finish()
    }
}

impl Display for PhlowTextView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.title.as_str())?;
        writeln!(f, "{}", "---------------------",)?;
        writeln!(f, "{}", self.compute_text())?;

        Ok(())
    }
}

impl PhlowView for PhlowTextView {
    fn get_title(&self) -> &str {
        self.title.as_str()
    }

    fn get_priority(&self) -> usize {
        self.priority
    }

    fn get_view_type(&self) -> &str {
        Self::view_type()
    }

    fn get_defining_method(&self) -> &PhlowViewMethod {
        &self.defining_method
    }

    fn view_type() -> &'static str
    where
        Self: Sized,
    {
        "text_view"
    }

    fn object(&self) -> &PhlowObject {
        &self.object
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    #[cfg(feature = "view-specification")]
    fn as_view_specification_builder(&self) -> &dyn crate::AsPhlowViewSpecification {
        self
    }
}

#[cfg(feature = "view-specification")]
mod specification {
    use serde::Serialize;

    use crate::{
        AsPhlowViewSpecification, PhlowViewSpecification, PhlowViewSpecificationDataTransport,
        PhlowViewSpecificationListingItem,
    };

    use super::*;

    #[derive(Debug, Clone, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PhlowTextViewSpecification {
        title: String,
        priority: usize,
        data_transport: PhlowViewSpecificationDataTransport,
        string: String,
        method_selector: String,
    }

    #[typetag::serialize(name = "GtPhlowTextEditorViewSpecification")]
    impl PhlowViewSpecification for PhlowTextViewSpecification {
        fn retrieve_items(&self) -> Vec<Box<dyn PhlowViewSpecificationListingItem>> {
            vec![]
        }

        fn retrieve_sent_item(&self, item: &PhlowObject) -> PhlowObject {
            item.clone()
        }
    }

    impl AsPhlowViewSpecification for PhlowTextView {
        fn create_view_specification(&self) -> Option<Box<dyn PhlowViewSpecification>> {
            Some(Box::new(PhlowTextViewSpecification {
                title: self.get_title().to_string(),
                priority: self.get_priority(),
                data_transport: PhlowViewSpecificationDataTransport::Included,
                string: self.compute_text(),
                method_selector: self.get_defining_method().full_method_name.clone(),
            }))
        }
    }
}
