use std::any::Any;
use std::fmt::{Debug, Display, Formatter};

use crate::reflection::AnyValue;
use crate::{PhlowObject, PhlowView, PhlowViewMethod};

#[allow(unused)]
pub struct PhlowListView {
    object: PhlowObject,
    defining_method: PhlowViewMethod,
    title: String,
    priority: usize,
    items_computation: Box<dyn Fn(&PhlowObject) -> Vec<PhlowObject>>,
    item_text_computation: Box<dyn Fn(&AnyValue, &PhlowObject) -> String>,
    send_computation: Box<dyn Fn(&AnyValue, &PhlowObject) -> Option<PhlowObject>>,
}

impl PhlowListView {
    pub fn new(object: PhlowObject, defining_method: PhlowViewMethod) -> Self {
        Self {
            object,
            defining_method,
            title: "".to_string(),
            priority: 10,
            items_computation: Box::new(|_object| Default::default()),
            item_text_computation: Box::new(|_item, object| object.to_string()),
            send_computation: Box::new(|_item, object| Some(object.clone())),
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

    pub fn items<T: 'static>(
        mut self,
        items_block: impl Fn(&T, &PhlowObject) -> Vec<PhlowObject> + 'static,
    ) -> Self {
        self.items_computation = Box::new(move |object: &PhlowObject| {
            // the type may differ when passing over ffi boundary...
            if let Some(reference) = object.value_ref() {
                items_block(reference, object)
            } else {
                vec![]
            }
        });
        self
    }

    pub fn item_text<T: 'static>(
        mut self,
        item_text_block: impl Fn(&T, &PhlowObject) -> String + 'static,
    ) -> Self {
        self.item_text_computation =
            Box::new(
                move |each, each_object: &PhlowObject| match each.as_ref_safe::<T>() {
                    Some(each) => item_text_block(each, each_object),
                    None => "Error coercing item type".to_string(),
                },
            );
        self
    }

    pub fn send<T: 'static>(
        mut self,
        item_send_block: impl Fn(&T, &PhlowObject) -> PhlowObject + 'static,
    ) -> Self {
        self.send_computation = Box::new(move |_each, object| {
            object
                .value_ref::<T>()
                .map(|item| item_send_block(item, object))
        });
        self
    }

    pub fn compute_items(&self) -> Vec<PhlowObject> {
        (self.items_computation)(&self.object)
    }

    pub fn compute_item_text(&self, item: &PhlowObject) -> String {
        (self.item_text_computation)(item.value(), item)
    }

    pub fn compute_item_send(&self, item: &PhlowObject) -> PhlowObject {
        ((self.send_computation)(item.value(), item)).unwrap_or_else(|| item.clone())
    }
}

impl Debug for PhlowListView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PhlowListView").finish()
    }
}

impl Display for PhlowListView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.title.as_str())?;
        writeln!(f, "{}", "---------------------",)?;
        writeln!(f, "{0: >3} | {1}", "i", "item",)?;

        let items = self.compute_items();

        for (index, item) in items.into_iter().enumerate() {
            writeln!(f, "{0:>3} | {1}", index, self.compute_item_text(&item))?;
        }

        Ok(())
    }
}

impl PhlowView for PhlowListView {
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

    fn view_type() -> &'static str {
        "list_view"
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
}
