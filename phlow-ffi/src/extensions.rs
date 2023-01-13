use phlow::{phlow, PhlowObject};
use value_box::ValueBox;

#[no_mangle]
pub extern "C" fn phlow_to_object_i32(number: i32) -> *mut ValueBox<PhlowObject> {
    ValueBox::new(phlow!(number)).into_raw()
}
