use std::any;
use std::any::type_name;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use crate::{
    get_debug_fmt_fn, get_display_fmt_fn, AnyValue, DebugFmtFn, DisplayFmtFn, Fmt, Phlow,
    PhlowObject, PhlowView,
};

#[derive(Clone)]
#[repr(C)]
pub struct PhlowExtension {
    view_methods_fn: Arc<dyn Fn(&PhlowExtension) -> Vec<PhlowViewMethod> + Send + Sync + 'static>,
    category: &'static str,
    target: &'static str,
}

impl PhlowExtension {
    pub fn new<Category, T: Phlow<Category> + 'static>() -> Self {
        Self {
            view_methods_fn: Arc::new(|extension| T::phlow_view_methods(extension)),
            category: any::type_name::<Category>(),
            target: any::type_name::<T>(),
        }
    }

    pub fn category_name(&self) -> &str {
        self.category
    }

    pub fn target_type_name(&self) -> &str {
        self.target
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
#[repr(C)]
pub struct PhlowViewMethod {
    pub method:
        Arc<dyn Fn(&PhlowObject, &PhlowViewMethod) -> Option<Box<dyn PhlowView>> + Send + Sync>,
    pub extension: PhlowExtension,
    pub method_name: String,
    pub full_method_name: String,
    pub source_code: String,
}

impl PhlowViewMethod {
    pub fn as_view(&self, object: &PhlowObject) -> Option<Box<dyn PhlowView>> {
        (self.method)(object, self)
    }

    pub fn source_code(&self) -> &str {
        self.source_code.as_str()
    }
}

impl Debug for PhlowViewMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.full_method_name.as_str())
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct PrintExtensions {
    display_fmt_fn: Option<DisplayFmtFn>,
    debug_fmt_fn: Option<DebugFmtFn>,
}

impl PrintExtensions {
    pub fn new<T: 'static>() -> Self {
        Self {
            display_fmt_fn: get_display_fmt_fn::<T>(),
            debug_fmt_fn: get_debug_fmt_fn::<T>(),
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

impl Debug for PrintExtensions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field(
                "display_fmt_fn",
                if self.display_fmt_fn.is_some() {
                    &"Some(...)"
                } else {
                    &"None"
                },
            )
            .field(
                "debug_fmt_fn",
                if self.debug_fmt_fn.is_some() {
                    &"Some(...)"
                } else {
                    &"None"
                },
            )
            .finish()
    }
}
