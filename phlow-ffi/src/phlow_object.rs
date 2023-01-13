use phlow::PhlowObject;
use string_box::StringBox;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub extern "C" fn phlow_object_to_string(
    phlow_object: *mut ValueBox<PhlowObject>,
    string: *mut ValueBox<StringBox>,
) {
    phlow_object
        .to_ref()
        .and_then(|phlow_object| {
            string.with_mut(|string| string.set_string(phlow_object.to_string()))
        })
        .log();
}

#[no_mangle]
pub extern "C" fn phlow_object_get_value_type(
    phlow_object: *mut ValueBox<PhlowObject>,
    value_type: *mut ValueBox<StringBox>,
) {
    phlow_object
        .to_ref()
        .and_then(|phlow_object| {
            value_type.with_mut(|value_type| value_type.set_string(phlow_object.value_type().to_string()))
        })
        .log();
}

#[no_mangle]
pub extern "C" fn phlow_object_drop(phlow_object: *mut ValueBox<PhlowObject>) {
    phlow_object.release();
}

#[no_mangle]
pub extern "C" fn phlow_object_vec_len(any_vec: *mut ValueBox<Vec<PhlowObject>>) -> usize {
    any_vec.with_ref(|any_vec| any_vec.len()).or_log(0)
}

#[no_mangle]
pub extern "C" fn phlow_object_vec_drop(any_vec: *mut ValueBox<Vec<PhlowObject>>) {
    any_vec.release();
}
