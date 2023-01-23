use crate::{PhlowObject, PhlowView, PhlowViewMethod, TypedPhlowObject};
use std::any::Any;
use std::fmt::{Debug, Display, Formatter};

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

    pub fn compute_text(&self) -> String {
        (self.text_computation)(&self.object)
    }
}

impl Debug for PhlowTextView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PhlowColumnListView").finish()
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
}
