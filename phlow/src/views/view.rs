use std::any::Any;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use crate::{PhlowListView, PhlowObject};

pub trait PhlowView: Debug + Display + Any {
    fn get_title(&self) -> &str;
    fn get_priority(&self) -> usize;
    fn get_view_type(&self) -> &str;
    fn view_type() -> &'static str
    where
        Self: Sized;
    fn object(&self) -> &PhlowObject;
    fn list(&self) -> PhlowListView {
        PhlowListView::new(self.object().clone())
    }
    fn as_any(&self) -> &dyn Any;
    fn to_any(self: Box<Self>) -> Box<dyn Any>;
}

#[derive(Debug)]
pub struct PhlowViewContext {}

#[derive(Debug)]
pub struct PhlowProtoView {
    object: PhlowObject,
}

impl PhlowProtoView {
    pub fn new(object: PhlowObject) -> Self {
        Self { object }
    }
}

impl Display for PhlowProtoView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ProtoView")
    }
}

impl PhlowView for PhlowProtoView {
    fn get_title(&self) -> &str {
        "Untitled"
    }

    fn get_priority(&self) -> usize {
        10
    }

    fn get_view_type(&self) -> &str {
        Self::view_type()
    }

    fn view_type() -> &'static str {
        "proto_view"
    }

    fn object(&self) -> &PhlowObject {
        &self.object
    }

    fn as_any(&self) -> &dyn Any {
        todo!()
    }

    fn to_any(self: Box<Self>) -> Box<dyn Any> {
        todo!()
    }
}

pub fn downcast_view_ref<T: PhlowView>(
    phlow_view: &Box<dyn PhlowView>,
) -> Result<&T, Box<dyn Error>> {
    if phlow_view.get_view_type() == T::view_type() {
        let view_any = phlow_view.as_any();
        let view = unsafe { &*(view_any as *const dyn Any as *const T) };
        Ok(view)
    } else {
        Err(Into::<Box<dyn Error>>::into(format!(
            "Failed to downcast {} to {}",
            phlow_view.get_view_type(),
            T::view_type()
        )))
    }
}
