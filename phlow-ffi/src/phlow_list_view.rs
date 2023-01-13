use string_box::StringBox;
use value_box::{BoxerError, ReturnBoxerResult, ValueBox, ValueBoxPointer};

use phlow::{downcast_view_ref, PhlowListView, PhlowObject, PhlowView};

#[no_mangle]
pub extern "C" fn phlow_list_view_compute_items(
    phlow_view: *mut ValueBox<Box<dyn PhlowView>>,
) -> *mut ValueBox<Vec<PhlowObject>> {
    phlow_view
        .to_ref()
        .and_then(|phlow_view| {
            downcast_view_ref::<PhlowListView>(&phlow_view)
                .map(|view| view.compute_items())
                .map_err(|error| error.into())
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
    phlow_view
        .to_ref()
        .and_then(|phlow_view| {
            downcast_view_ref::<PhlowListView>(&phlow_view)
                .map_err(|error| error.into())
                .map(|list_view| {
                    items.with_ref(|items| {
                        items.get(index).map(|item| {
                            item_text.with_mut(|item_text| {
                                item_text.set_string(list_view.compute_item_text(item))
                            })
                        })
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
    phlow_view
        .to_ref()
        .and_then(|phlow_view| {
            downcast_view_ref::<PhlowListView>(&phlow_view)
                .map_err(|error| error.into())
                .and_then(|list_view| {
                    items.to_ref().and_then(|items| {
                        items
                            .get(index)
                            .ok_or_else(|| {
                                BoxerError::AnyError(
                                    format!("Item at {} does not exist", index).into(),
                                )
                            })
                            .map(|item| list_view.compute_item_send(item))
                    })
                })
        })
        .into_raw()
}
