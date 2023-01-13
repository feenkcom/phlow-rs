use phlow::PhlowViewMethod;
use string_box::StringBox;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub extern "C" fn phlow_view_method_get_full_name(
    phlow_method: *mut ValueBox<PhlowViewMethod>,
    full_name: *mut ValueBox<StringBox>,
) {
    phlow_method
        .to_ref()
        .and_then(|phlow_method| {
            full_name
                .with_mut(|full_name| full_name.set_string(phlow_method.full_method_name.clone()))
        })
        .log();
}

#[no_mangle]
pub extern "C" fn phlow_view_method_get_name(
    phlow_method: *mut ValueBox<PhlowViewMethod>,
    name: *mut ValueBox<StringBox>,
) {
    phlow_method
        .to_ref()
        .and_then(|phlow_method| {
            name.with_mut(|name| name.set_string(phlow_method.method_name.clone()))
        })
        .log();
}

#[no_mangle]
pub extern "C" fn phlow_view_method_drop(phlow_method: *mut ValueBox<PhlowViewMethod>) {
    phlow_method.release();
}
