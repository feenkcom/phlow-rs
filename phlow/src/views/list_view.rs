use std::any::Any;
use std::fmt::{Debug, Display, Formatter};

use crate::{PhlowObject, PhlowView, PhlowViewMethod, TypedPhlowObject};

#[allow(unused)]
pub struct PhlowListView {
    object: PhlowObject,
    defining_method: PhlowViewMethod,
    title: String,
    priority: usize,
    items_computation: Box<dyn Fn(&PhlowObject) -> Vec<PhlowObject>>,
    item_text_computation: Box<dyn Fn(&PhlowObject) -> String>,
    send_computation: Box<dyn Fn(&PhlowObject) -> Option<PhlowObject>>,
}

impl PhlowListView {
    pub fn new(object: PhlowObject, defining_method: PhlowViewMethod) -> Self {
        Self {
            object,
            defining_method,
            title: "".to_string(),
            priority: 10,
            items_computation: Box::new(|_object| Default::default()),
            item_text_computation: Box::new(|object| object.to_string()),
            send_computation: Box::new(|object| Some(object.clone())),
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
        items_block: impl Fn(TypedPhlowObject<T>) -> Vec<PhlowObject> + 'static,
    ) -> Self {
        self.items_computation = Box::new(move |each_object: &PhlowObject| {
            // the type may differ when passing over ffi boundary...
            if let Some(each_reference) = each_object.value_ref::<T>() {
                items_block(TypedPhlowObject::new(each_object, &each_reference))
            } else {
                vec![]
            }
        });
        self
    }

    pub fn item_text<T: 'static>(
        mut self,
        item_text_block: impl Fn(TypedPhlowObject<T>) -> String + 'static,
    ) -> Self {
        self.item_text_computation =
            Box::new(move |each_object| match each_object.value_ref::<T>() {
                Some(each_reference) => {
                    item_text_block(TypedPhlowObject::new(each_object, &each_reference))
                }
                None => "Error coercing item type".to_string(),
            });
        self
    }

    pub fn send<T: 'static>(
        mut self,
        item_send_block: impl Fn(TypedPhlowObject<T>) -> PhlowObject + 'static,
    ) -> Self {
        self.send_computation = Box::new(move |each_object| {
            each_object.value_ref::<T>().map(|each_reference| {
                item_send_block(TypedPhlowObject::new(each_object, &each_reference))
            })
        });
        self
    }

    pub fn compute_items(&self) -> Vec<PhlowObject> {
        (self.items_computation)(&self.object)
    }

    pub fn compute_item_text(&self, item: &PhlowObject) -> String {
        (self.item_text_computation)(item)
    }

    pub fn compute_item_send(&self, item: &PhlowObject) -> PhlowObject {
        ((self.send_computation)(item)).unwrap_or_else(|| item.clone())
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
