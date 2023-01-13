use string_box::StringBox;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

use phlow::PhlowView;

#[no_mangle]
pub extern "C" fn phlow_view_get_type(
    phlow_view: *mut ValueBox<Box<dyn PhlowView>>,
    view_type: *mut ValueBox<StringBox>,
) {
    phlow_view
        .to_ref()
        .and_then(|phlow_view| {
            view_type
                .with_mut(|view_type| view_type.set_string(phlow_view.get_view_type().to_string()))
        })
        .log();
}

#[no_mangle]
pub extern "C" fn phlow_view_get_title(
    phlow_view: *mut ValueBox<Box<dyn PhlowView>>,
    view_title: *mut ValueBox<StringBox>,
) {
    phlow_view
        .to_ref()
        .and_then(|phlow_view| {
            view_title
                .with_mut(|view_title| view_title.set_string(phlow_view.get_title().to_string()))
        })
        .log();
}

#[no_mangle]
pub extern "C" fn phlow_view_get_priority(phlow_view: *mut ValueBox<Box<dyn PhlowView>>) -> usize {
    phlow_view
        .with_ref(|phlow_view| phlow_view.get_priority())
        .or_log(0)
}

#[no_mangle]
pub extern "C" fn phlow_view_drop(phlow_view: *mut ValueBox<Box<dyn PhlowView>>) {
    phlow_view.release();
}
