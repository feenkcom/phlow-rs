// printing.rs requires #specialization to detect if type implements Debug or Display
// to provide some printing capabilities to all types.
#![cfg_attr(feature = "printing", feature(specialization))]

#[cfg(feature = "phlow-derive")]
pub use phlow_derive::{extensions, view};

pub use crate::meta::*;
pub use crate::object::*;
pub use crate::printing::*;
pub use crate::reflection::*;
pub use crate::views::*;

mod meta;
mod object;
mod reflection;
mod views;
mod printing;

pub trait Phlow<Category> {
    fn phlow_view_methods(extension: &PhlowExtension) -> Vec<PhlowViewMethod>;
    fn phlow_extension(&self) -> Option<PhlowExtension>;
}

impl<Category, T> Phlow<Category> for T {
    default fn phlow_view_methods(_extension: &PhlowExtension) -> Vec<PhlowViewMethod> {
        vec![]
    }
    default fn phlow_extension(&self) -> Option<PhlowExtension> {
        None
    }
}

#[macro_export]
macro_rules! phlow {
    ($var:expr) => {{
        phlow::PhlowObject::object($var, crate::phlow_extensions)
    }};
    (&$var:expr, $parent:expr) => {{
        phlow::PhlowObject::reference(&$var, $parent, crate::phlow_extensions)
    }};
    ($var:expr, $parent:expr) => {{
        phlow::PhlowObject::reference($var, $parent, crate::phlow_extensions)
    }};
}

#[macro_export]
macro_rules! phlow_all {
    ($iter:expr) => {{
        $iter
            .into_iter()
            .map(|each| phlow!(each))
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
        pub(crate) fn phlow_extensions<T: 'static>(value: &T) -> Vec<phlow::PhlowExtension> {
            let mut extensions = vec![];
            $(
                if let Some(extension) = phlow::Phlow::<$es>::phlow_extension(value) {
                    extensions.push(extension);
                }
            )*
            extensions
        }
        pub(crate) fn phlow_view_methods<T: 'static>(value: &T) -> Vec<phlow::PhlowViewMethod> {
            phlow_extensions(value)
                .into_iter()
                .map(|extension| extension.view_methods())
                .flatten()
                .collect()
        }
    };
}
