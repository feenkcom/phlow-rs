use phlow::PhlowViewMethod;
use string_box::StringBox;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub extern "C" fn phlow_view_method_get_full_name(
    phlow_method: *mut ValueBox<PhlowViewMethod>,
    full_name: *mut ValueBox<StringBox>,
) {
    phlow_method
        .with_ref(|phlow_method| {
            full_name.with_mut_ok(|full_name| {
                full_name.set_string(phlow_method.full_method_name.clone())
            })
        })
        .log();
}

#[no_mangle]
pub extern "C" fn phlow_view_method_get_name(
    phlow_method: *mut ValueBox<PhlowViewMethod>,
    name: *mut ValueBox<StringBox>,
) {
    phlow_method
        .with_ref(|phlow_method| {
            name.with_mut_ok(|name| name.set_string(phlow_method.method_name.clone()))
        })
        .log();
}

#[no_mangle]
pub extern "C" fn phlow_view_method_drop(phlow_method: *mut ValueBox<PhlowViewMethod>) {
    phlow_method.release();
}

#[no_mangle]
pub extern "C" fn phlow_view_methods_pop(
    phlow_methods: *mut ValueBox<Vec<PhlowViewMethod>>,
) -> *mut ValueBox<PhlowViewMethod> {
    phlow_methods
        .with_mut_ok(|phlow_methods| phlow_methods.pop())
        .map(|method| {
            method.map_or(std::ptr::null_mut(), |method| {
                ValueBox::new(method).into_raw()
            })
        })
        .or_log(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn phlow_view_methods_drop(phlow_methods: *mut ValueBox<Vec<PhlowViewMethod>>) {
    phlow_methods.release();
}
