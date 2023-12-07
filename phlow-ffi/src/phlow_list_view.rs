use phlow::{PhlowListView, PhlowObject, PhlowView};
use string_box::StringBox;
use value_box::{BoxerError, ReturnBoxerResult, ValueBox, ValueBoxIntoRaw, ValueBoxPointer};

use crate::with_view;

#[no_mangle]
pub extern "C" fn phlow_list_view_compute_items(
    phlow_view: *mut ValueBox<Box<dyn PhlowView>>,
) -> *mut ValueBox<Vec<PhlowObject>> {
    with_view(phlow_view, |phlow_view: &PhlowListView| {
        Ok(ValueBox::new(phlow_view.compute_items()))
    })
    .into_raw()
}

#[no_mangle]
pub extern "C" fn phlow_list_view_compute_item_text_at(
    phlow_view: *mut ValueBox<Box<dyn PhlowView>>,
    items: *mut ValueBox<Vec<PhlowObject>>,
    index: usize,
    item_text: *mut ValueBox<StringBox>,
) {
    with_view(phlow_view, |phlow_view: &PhlowListView| {
        items.with_ref(|items| {
            items
                .get(index)
                .ok_or_else(
                    || BoxerError::AnyError(format!("Item at {} does not exist", index).into())
                )
                .map(|item| {
                    item_text.with_mut_ok(
                        |item_text| item_text.set_string(phlow_view.compute_item_text(item))
                    )
                })
        })
    })
    .log();
}

#[no_mangle]
pub extern "C" fn phlow_list_view_compute_item_send_at(
    phlow_view: *mut ValueBox<Box<dyn PhlowView>>,
    items: *mut ValueBox<Vec<PhlowObject>>,
    index: usize,
) -> *mut ValueBox<PhlowObject> {
    with_view(phlow_view, |phlow_view: &PhlowListView| {
        items.with_ref(|items| {
            items
                .get(index)
                .ok_or_else(
                    || BoxerError::AnyError(format!("Item at {} does not exist", index).into())
                )
                .map(|item| ValueBox::new(phlow_view.compute_item_send(item)))
        })
    })
    .into_raw()
}
