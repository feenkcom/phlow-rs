use std::any::Any;
use string_box::StringBox;
use value_box::{Result, ReturnBoxerResult, ValueBox, ValueBoxPointer};

use phlow::{downcast_view_ref, PhlowView};

pub fn with_view<T: PhlowView, R: Any>(
    phlow_view: *mut ValueBox<Box<dyn PhlowView>>,
    op: impl FnOnce(&T) -> Result<R>,
) -> Result<R> {
    phlow_view.with_ref(|phlow_view| {
        downcast_view_ref::<T>(&phlow_view)
            .map_err(|error| error.into())
            .and_then(|view| op(view))
    })
}

#[no_mangle]
pub extern "C" fn phlow_view_get_type(
    phlow_view: *mut ValueBox<Box<dyn PhlowView>>,
    view_type: *mut ValueBox<StringBox>,
) {
    phlow_view
        .with_ref(|phlow_view| {
            view_type.with_mut_ok(
                |view_type| view_type.set_string(phlow_view.get_view_type().to_string())
            )
        })
        .log();
}

#[no_mangle]
pub extern "C" fn phlow_view_get_title(
    phlow_view: *mut ValueBox<Box<dyn PhlowView>>,
    view_title: *mut ValueBox<StringBox>,
) {
    phlow_view
        .with_ref(|phlow_view| {
            view_title
                .with_mut_ok(|view_title| view_title.set_string(phlow_view.get_title().to_string()))
        })
        .log();
}

#[no_mangle]
pub extern "C" fn phlow_view_get_priority(phlow_view: *mut ValueBox<Box<dyn PhlowView>>) -> usize {
    phlow_view
        .with_ref_ok(|phlow_view| phlow_view.get_priority())
        .or_log(0)
}

#[no_mangle]
pub extern "C" fn phlow_view_get_source_code(
    phlow_view: *mut ValueBox<Box<dyn PhlowView>>,
    source_code: *mut ValueBox<StringBox>,
) {
    phlow_view
        .with_ref(|phlow_view| {
            source_code.with_mut_ok(|source_code| {
                source_code.set_string(phlow_view.get_defining_method().source_code().to_string())
            })
        })
        .log();
}

#[no_mangle]
pub extern "C" fn phlow_view_drop(phlow_view: *mut ValueBox<Box<dyn PhlowView>>) {
    phlow_view.release();
}

#[no_mangle]
pub extern "C" fn phlow_views_pop(
    phlow_views: *mut ValueBox<Vec<Box<dyn PhlowView>>>,
) -> *mut ValueBox<Box<dyn PhlowView>> {
    phlow_views
        .with_mut_ok(|phlow_views| phlow_views.pop())
        .map(|view| view.map_or(std::ptr::null_mut(), |view| ValueBox::new(view).into_raw()))
        .or_log(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn phlow_views_drop(phlow_views: *mut ValueBox<Vec<Box<dyn PhlowView>>>) {
    phlow_views.release();
}
