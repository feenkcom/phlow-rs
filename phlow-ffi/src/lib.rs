#![feature(specialization)]
#![allow(non_snake_case)]

#[macro_use]
extern crate phlow;

use string_box::StringBox;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

use phlow::{PhlowObject, PhlowView, PhlowViewMethod};
use phlow_extensions::CoreExtensions;

pub mod extensions;
pub mod phlow_list_view;
pub mod phlow_object;
pub mod phlow_view;
pub mod phlow_view_method;

import_extensions!(CoreExtensions);

#[no_mangle]
pub fn phlow_init_env_logger() {
    env_logger::init();
}

#[no_mangle]
pub extern "C" fn phlow_object_get_view_methods(
    phlow_object: *mut ValueBox<PhlowObject>,
) -> *mut ValueBox<Vec<PhlowViewMethod>> {
    phlow_object
        .with_ref(|phlow_object| phlow_object.phlow_view_methods())
        .into_raw()
}

#[no_mangle]
pub extern "C" fn phlow_object_get_views(
    phlow_object: *mut ValueBox<PhlowObject>,
) -> *mut ValueBox<Vec<Box<dyn PhlowView>>> {
    phlow_object
        .with_ref(|phlow_object| phlow_object.phlow_views())
        .into_raw()
}

#[no_mangle]
pub extern "C" fn phlow_object_get_view_named(
    phlow_object: *mut ValueBox<PhlowObject>,
    view_name: *mut ValueBox<StringBox>,
) -> *mut ValueBox<Box<dyn PhlowView>> {
    phlow_object
        .to_ref()
        .and_then(|phlow_object| {
            view_name.with_ref(|view_name| {
                phlow_object
                    .phlow_view_named(view_name.as_str())
                    .map_or(std::ptr::null_mut(), |view| ValueBox::new(view).into_raw())
            })
        })
        .or_log(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn phlow_view_methods_pop(
    phlow_methods: *mut ValueBox<Vec<PhlowViewMethod>>,
) -> *mut ValueBox<PhlowViewMethod> {
    phlow_methods
        .with_mut(|phlow_methods| phlow_methods.pop())
        .map(|method| {
            method.map_or(std::ptr::null_mut(), |method| {
                ValueBox::new(method).into_raw()
            })
        })
        .or_log(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn phlow_views_pop(
    phlow_views: *mut ValueBox<Vec<Box<dyn PhlowView>>>,
) -> *mut ValueBox<Box<dyn PhlowView>> {
    phlow_views
        .with_mut(|phlow_views| phlow_views.pop())
        .map(|view| view.map_or(std::ptr::null_mut(), |view| ValueBox::new(view).into_raw()))
        .or_log(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn phlow_view_methods_drop(phlow_methods: *mut ValueBox<Vec<PhlowViewMethod>>) {
    phlow_methods.release();
}

#[no_mangle]
pub extern "C" fn phlow_views_drop(phlow_views: *mut ValueBox<Vec<Box<dyn PhlowView>>>) {
    phlow_views.release();
}
