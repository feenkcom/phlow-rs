use phlow::{PhlowTextView, PhlowView};
use string_box::StringBox;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

use crate::with_view;

#[no_mangle]
pub extern "C" fn phlow_text_view_compute_text(
    phlow_view: *mut ValueBox<Box<dyn PhlowView>>,
    text: *mut ValueBox<StringBox>,
) {
    with_view(phlow_view, |phlow_view: &PhlowTextView| {
        text.with_mut_ok(|text| text.set_string(phlow_view.compute_text()))
    })
    .log();
}
