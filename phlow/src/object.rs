use std::any::type_name;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use crate::{AnyValue, PhlowExtension, PhlowView, PhlowViewMethod, PrintExtensions};

#[derive(Clone)]
pub struct PhlowObject(Rc<PhlowObjectData>);
struct PhlowObjectData {
    // to make sure that when we browse a reference, it stays alive as long as the previous inspector is alive
    parent: Option<PhlowObject>,
    // when value is reference - the previous inspector must be initialized
    value: AnyValue,
    // the full type path
    value_type: &'static str,
    // extensions are simple vtable that know functions used to get specific extensions
    phlow_extensions: Vec<PhlowExtension>,
    // detects available printable options such as Display, Debug etc..
    print_extensions: PrintExtensions,
}

impl PhlowObject {
    pub fn object<T: 'static>(
        object: T,
        phlow_extensions_fn: impl Fn(&T) -> Vec<PhlowExtension> + 'static,
    ) -> Self {
        let extensions = phlow_extensions_fn(&object);
        let print_extensions = PrintExtensions::new(&object);
        Self::new(
            None,
            AnyValue::object(object),
            type_name::<T>(),
            extensions,
            print_extensions,
        )
    }

    pub fn reference<T: 'static>(
        object: &T,
        parent: &PhlowObject,
        phlow_extensions_fn: impl Fn(&T) -> Vec<PhlowExtension> + 'static,
    ) -> Self {
        let extensions = phlow_extensions_fn(object);
        let print_extensions = PrintExtensions::new(object);
        Self::new(
            Some(parent.clone()),
            AnyValue::reference(object),
            type_name::<T>(),
            extensions,
            print_extensions,
        )
    }

    fn new(
        parent: Option<PhlowObject>,
        value: AnyValue,
        value_type: &'static str,
        phlow_extensions: Vec<PhlowExtension>,
        print_extensions: PrintExtensions,
    ) -> Self {
        Self(Rc::new(PhlowObjectData {
            parent,
            value,
            value_type,
            phlow_extensions,
            print_extensions,
        }))
    }

    pub fn to_string(&self) -> String {
        self.0.print_extensions.to_string(self.value())
    }

    pub fn value(&self) -> &AnyValue {
        &self.0.value
    }

    pub fn value_ref<T: 'static>(&self) -> &T {
        self.value().as_ref()
    }

    pub fn value_type(&self) -> &str {
        self.0.value_type
    }

    pub fn parent(&self) -> Option<&PhlowObject> {
        self.0.parent.as_ref()
    }

    pub fn phlow_extensions(&self) -> &Vec<PhlowExtension> {
        &self.0.phlow_extensions
    }

    pub fn phlow_view_methods(&self) -> Vec<PhlowViewMethod> {
        self.phlow_extensions()
            .into_iter()
            .map(|extension| extension.view_methods())
            .flatten()
            .collect()
    }

    pub fn phlow_view_named(&self, name: impl AsRef<str>) -> Option<Box<dyn PhlowView>> {
        let target_name: &str = name.as_ref();

        self.phlow_view_methods()
            .into_iter()
            .find(|each_method| each_method.method_name.as_str() == target_name)
            .map(|each_method| each_method.as_view(&self))
    }

    pub fn phlow_views(&self) -> Vec<Box<dyn PhlowView>> {
        self.phlow_view_methods()
            .into_iter()
            .map(|each_method| each_method.as_view(&self))
            .collect()
    }
}

impl Debug for PhlowObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("extensions", &self.phlow_view_methods())
            .finish()
    }
}
