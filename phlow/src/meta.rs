use std::any;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use crate::{
    get_debug_fmt_fn, get_display_fmt_fn, AnyValue, DebugFmtFn, DisplayFmtFn, Fmt, Phlow,
    PhlowObject, PhlowView,
};

#[derive(Clone)]
pub struct PhlowExtension {
    view_methods_fn: Rc<dyn Fn(&PhlowExtension) -> Vec<PhlowViewMethod>>,
    category: &'static str,
    target: &'static str,
}

impl PhlowExtension {
    pub fn new<Category, T: Phlow<Category> + 'static>() -> Self {
        Self {
            view_methods_fn: Rc::new(|extension| T::phlow_view_methods(extension)),
            category: any::type_name::<Category>(),
            target: any::type_name::<T>(),
        }
    }

    pub fn view_methods(&self) -> Vec<PhlowViewMethod> {
        (self.view_methods_fn)(self)
    }
}

impl Debug for PhlowExtension {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(any::type_name::<Self>())
            .field("category", &self.category)
            .field("target", &self.target)
            .finish()
    }
}

#[derive(Clone)]
pub struct PhlowViewMethod {
    pub method: Rc<dyn Fn(&PhlowObject) -> Box<dyn PhlowView>>,
    pub extension: PhlowExtension,
    pub method_name: String,
    pub full_method_name: String,
}

impl PhlowViewMethod {
    pub fn as_view(&self, object: &PhlowObject) -> Box<dyn PhlowView> {
        (self.method)(object)
    }
}

impl Debug for PhlowViewMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.full_method_name.as_str())
    }
}

#[derive(Clone)]
pub struct PrintExtensions {
    display_fmt_fn: Option<DisplayFmtFn>,
    debug_fmt_fn: Option<DebugFmtFn>,
}

impl PrintExtensions {
    pub fn new<T: 'static>(value: &T) -> Self {
        Self {
            display_fmt_fn: get_display_fmt_fn(value),
            debug_fmt_fn: get_debug_fmt_fn(value),
        }
    }

    pub fn to_string(&self, value: &AnyValue) -> String {
        self.display_string(value)
            .or_else(|| self.debug_string(value))
            .unwrap_or_else(|| "Doesn't support Display or Debug".to_string())
    }

    pub fn debug_string(&self, value: &AnyValue) -> Option<String> {
        self.debug_fmt_fn
            .as_ref()
            .map(|func| format!("{:?}", &Fmt(|f| func(value, f))))
    }

    pub fn display_string(&self, value: &AnyValue) -> Option<String> {
        self.display_fmt_fn
            .as_ref()
            .map(|func| format!("{}", &Fmt(|f| func(value, f))))
    }
}
