// printing.rs requires #specialization to detect if type implements Debug or Display
// to provide some printing capabilities to all types.
#![allow(incomplete_features)]
#![cfg_attr(feature = "printing", feature(specialization))]

#[cfg(feature = "phlow-derive")]
pub use phlow_derive::{extensions, view};

pub extern crate log;

pub use crate::meta::*;
pub use crate::object::*;
pub use crate::printing::*;
pub use crate::reflection::*;
pub use crate::views::*;

mod meta;
mod object;
mod printing;
mod reflection;
mod views;

pub trait Phlow<Category> {
    fn phlow_view_methods(extension: &PhlowExtension) -> Vec<PhlowViewMethod>;
    fn phlow_extension() -> Option<PhlowExtension>;
}

impl<Category, T> Phlow<Category> for T {
    default fn phlow_view_methods(_extension: &PhlowExtension) -> Vec<PhlowViewMethod> {
        vec![]
    }
    default fn phlow_extension() -> Option<PhlowExtension> {
        None
    }
}

#[macro_export]
macro_rules! phlow {
    ($var:expr) => {{
        if let Some(phlow_object) = phlow::AsPhlowObject::try_into_phlow_object(&$var) {
            phlow_object.clone()
        }
        else {
            phlow::PhlowObject::object($var, crate::phlow_extensions_of_val)
        }
    }};
    ($var:expr, <$($generic_type:ident),+>) => {{
        if let Some(phlow_object) = phlow::AsPhlowObject::try_into_phlow_object(&$var) {
            phlow_object.clone()
        }
        else {
            phlow::PhlowObject::object_with_generics(
                $var,
                crate::phlow_extensions_of_val,
                vec![
                    $(
                        phlow::PhlowType::new::<$generic_type>(crate::phlow_extensions::<$generic_type>)
                    )+
                ])
        }
    }};
    (&$var:expr, $parent:expr) => {{
        phlow::PhlowObject::reference(&$var, $parent, crate::phlow_extensions_of_val)
    }};
    ($var:expr, $parent:expr) => {{
        phlow::PhlowObject::reference($var, $parent, crate::phlow_extensions_of_val)
    }};
}

#[macro_export]
macro_rules! phlow_ref {
    ($var:expr) => {{
        if let Some(phlow_object) = phlow::AsPhlowObject::try_into_phlow_object($var) {
            phlow_object.clone()
        } else {
            phlow::PhlowObject::object($var.clone(), crate::phlow_extensions_of_val)
        }
    }};
    ($var:expr, $parent:expr) => {{
        phlow::PhlowObject::reference($var, &$parent.clone(), crate::phlow_extensions_of_val)
    }};
}

#[macro_export]
macro_rules! phlow_generic {
    ($child:expr, $parent:expr) => {{
        phlow::PhlowObject::construct_reference(
            $child,
            $parent
                .generic_phlow_type(0)
                .unwrap_or_else(|| phlow_type!($child)),
            Some($parent.clone()),
        )
    }};
}

#[macro_export]
macro_rules! phlow_type {
    ($var:expr) => {{
        phlow::PhlowType::of($var, crate::phlow_extensions_of_val)
    }};
}

#[macro_export]
macro_rules! phlow_all {
    ($iter:expr) => {{
        $iter
            .into_iter()
            .map(|each| phlow::phlow!(each))
            .collect::<Vec<phlow::PhlowObject>>()
    }};
}

#[macro_export]
macro_rules! define_extensions {
    ($e:ident) => {
        pub struct $e;
    };
}

#[macro_export]
macro_rules! import_extensions {
    ($($es:ident),*) => {
        pub(crate) fn phlow_extensions_of_val<T: 'static>(_value: &T) -> Vec<phlow::PhlowExtension> {
            phlow_extensions::<T>()
        }
        pub(crate) fn phlow_extensions<T: 'static>() -> Vec<phlow::PhlowExtension> {
            let mut extensions = vec![];
            $(
                if let Some(extension) = <T as phlow::Phlow::<$es>>::phlow_extension() {
                    extensions.push(extension);
                }
            )*
            extensions
        }

        #[inline]
        pub (crate) fn phlow_type<T: 'static>() -> phlow::PhlowType {
            phlow::PhlowType::new::<T>(phlow_extensions::<T>)
        }

        #[inline]
        pub (crate) fn phlow_type_fn_of_val<T: 'static>(_value: &T) -> fn () -> phlow::PhlowType {
            phlow_type::<T>
        }

        pub(crate) fn phlow_view_methods<T: 'static>(value: &T) -> Vec<phlow::PhlowViewMethod> {
            phlow_extensions_of_val(value)
                .into_iter()
                .map(|extension| extension.view_methods())
                .flatten()
                .collect()
        }
    };
}
