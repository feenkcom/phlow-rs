use std::any::{type_name, TypeId};
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
    // meta description of the type with the necessary vtables
    phlow_type: PhlowType,
    generic_types: Vec<PhlowType>,
}

impl PhlowObject {
    pub fn object<T: 'static>(
        object: T,
        phlow_extensions_fn: impl Fn(&T) -> Vec<PhlowExtension> + 'static,
    ) -> Self {
        let phlow_type = PhlowType::new::<T>(|| phlow_extensions_fn(&object));
        Self::new(AnyValue::object(object), phlow_type, vec![], None)
    }

    pub fn object_with_generics<T: 'static>(
        object: T,
        phlow_extensions_fn: impl Fn(&T) -> Vec<PhlowExtension> + 'static,
        generic_types: Vec<PhlowType>,
    ) -> Self {
        let phlow_type = PhlowType::new::<T>(|| phlow_extensions_fn(&object));
        Self::new(AnyValue::object(object), phlow_type, generic_types, None)
    }

    pub fn reference<T: 'static>(
        object: &T,
        parent: &PhlowObject,
        phlow_extensions_fn: impl Fn(&T) -> Vec<PhlowExtension> + 'static,
    ) -> Self {
        let phlow_type = PhlowType::new::<T>(|| phlow_extensions_fn(object));
        Self::new(
            AnyValue::reference(object),
            phlow_type,
            vec![],
            Some(parent.clone()),
        )
    }

    pub fn construct_reference<T: 'static>(
        reference: &T,
        phlow_type: PhlowType,
        parent: Option<PhlowObject>,
    ) -> Self {
        Self::new(AnyValue::reference(reference), phlow_type, vec![], parent)
    }

    fn new(
        value: AnyValue,
        phlow_type: PhlowType,
        generic_types: Vec<PhlowType>,
        parent: Option<PhlowObject>,
    ) -> Self {
        Self(Rc::new(PhlowObjectData {
            parent,
            value,
            phlow_type,
            generic_types,
        }))
    }

    pub fn phlow_type(&self) -> &PhlowType {
        &self.0.phlow_type
    }

    pub fn generic_phlow_type(&self, index: usize) -> Option<PhlowType> {
        self.0.generic_types.get(index).cloned()
    }

    pub fn to_string(&self) -> String {
        self.0.phlow_type.value_to_string(self.value())
    }

    pub fn value(&self) -> &AnyValue {
        &self.0.value
    }

    pub fn value_ref<T: 'static>(&self) -> Option<&T> {
        self.value().as_ref_safe()
    }

    pub fn value_type_name(&self) -> &str {
        self.0.phlow_type.type_name()
    }

    pub fn parent(&self) -> Option<&PhlowObject> {
        self.0.parent.as_ref()
    }

    pub fn phlow_view_methods(&self) -> Vec<PhlowViewMethod> {
        self.0
            .phlow_type
            .phlow_extensions
            .iter()
            .map(|extension| extension.view_methods())
            .flatten()
            .collect()
    }

    pub fn phlow_view_named(&self, name: impl AsRef<str>) -> Option<Box<dyn PhlowView>> {
        let target_name: &str = name.as_ref();

        self.phlow_view_methods()
            .into_iter()
            .find(|each_method| each_method.method_name.as_str() == target_name)
            .and_then(|each_method| each_method.as_view(&self))
    }

    pub fn phlow_views(&self) -> Vec<Box<dyn PhlowView>> {
        self.phlow_view_methods()
            .into_iter()
            .map(|each_method| each_method.as_view(&self))
            .filter(|each_view| each_view.is_some())
            .map(|each_view| each_view.unwrap())
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

#[derive(Debug, Clone)]
#[repr(C)]
pub struct PhlowType {
    // the full type path as a string
    type_name: &'static str,
    type_id: TypeId,
    // extensions are simple vtable that know functions used to get specific extensions
    phlow_extensions: Vec<PhlowExtension>,
    // detects available printable options such as Display, Debug etc..
    print_extensions: PrintExtensions,
}

impl PhlowType {
    pub fn of<T: 'static>(
        value: &T,
        phlow_extensions_fn: impl Fn(&T) -> Vec<PhlowExtension> + 'static,
    ) -> Self {
        Self::new::<T>(|| phlow_extensions_fn(value))
    }

    pub fn new<T: 'static>(phlow_extensions_fn: impl Fn() -> Vec<PhlowExtension>) -> Self {
        let phlow_extensions = phlow_extensions_fn();
        let print_extensions = PrintExtensions::new::<T>();
        Self {
            type_name: type_name::<T>(),
            type_id: TypeId::of::<T>(),
            phlow_extensions,
            print_extensions,
        }
    }

    pub fn type_name(&self) -> &str {
        self.type_name
    }

    pub fn value_to_string(&self, value: &AnyValue) -> String {
        self.print_extensions.to_string(value)
    }
}
