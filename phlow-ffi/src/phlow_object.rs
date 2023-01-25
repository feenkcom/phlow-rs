use phlow::{PhlowObject, PhlowView, PhlowViewMethod};
use string_box::StringBox;
use value_box::{BoxerError, ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub extern "C" fn phlow_object_get_view_methods(
    phlow_object: *mut ValueBox<PhlowObject>,
) -> *mut ValueBox<Vec<PhlowViewMethod>> {
    phlow_object
        .with_ref_ok(|phlow_object| phlow_object.phlow_view_methods())
        .into_raw()
}

#[no_mangle]
pub extern "C" fn phlow_object_get_views(
    phlow_object: *mut ValueBox<PhlowObject>,
) -> *mut ValueBox<Vec<Box<dyn PhlowView>>> {
    phlow_object
        .with_ref_ok(|phlow_object| phlow_object.phlow_views())
        .into_raw()
}

#[no_mangle]
pub extern "C" fn phlow_object_get_view_named(
    phlow_object: *mut ValueBox<PhlowObject>,
    view_name: *mut ValueBox<StringBox>,
) -> *mut ValueBox<Box<dyn PhlowView>> {
    phlow_object
        .with_ref(|phlow_object| {
            view_name.with_ref(|view_name| {
                phlow_object
                    .phlow_view_named(view_name.as_str())
                    .ok_or_else(|| {
                        BoxerError::AnyError(
                            format!("View named {} does not exist", view_name.as_str()).into(),
                        )
                    })
            })
        })
        .into_raw()
}

#[no_mangle]
pub extern "C" fn phlow_object_to_string(
    phlow_object: *mut ValueBox<PhlowObject>,
    string: *mut ValueBox<StringBox>,
) {
    phlow_object
        .with_ref(|phlow_object| {
            string.with_mut_ok(|string| string.set_string(phlow_object.to_string()))
        })
        .log();
}

#[no_mangle]
pub extern "C" fn phlow_object_get_value_type(
    phlow_object: *mut ValueBox<PhlowObject>,
    value_type: *mut ValueBox<StringBox>,
) {
    phlow_object
        .with_ref(|phlow_object| {
            value_type.with_mut_ok(|value_type| {
                value_type.set_string(phlow_object.value_type_name().to_string())
            })
        })
        .log();
}

#[no_mangle]
pub extern "C" fn phlow_object_drop(phlow_object: *mut ValueBox<PhlowObject>) {
    phlow_object.release();
}

#[no_mangle]
pub extern "C" fn phlow_object_vec_len(any_vec: *mut ValueBox<Vec<PhlowObject>>) -> usize {
    any_vec.with_ref_ok(|any_vec| any_vec.len()).or_log(0)
}

#[no_mangle]
pub extern "C" fn phlow_object_vec_drop(any_vec: *mut ValueBox<Vec<PhlowObject>>) {
    any_vec.release();
}
