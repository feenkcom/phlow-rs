use phlow::{PhlowColumnedListView, PhlowObject, PhlowView};
use string_box::StringBox;
use value_box::{BoxerError, ReturnBoxerResult, ValueBox, ValueBoxIntoRaw, ValueBoxPointer};

use crate::with_view;

#[no_mangle]
pub extern "C" fn phlow_columned_list_view_compute_items(
    phlow_view: *mut ValueBox<Box<dyn PhlowView>>,
) -> *mut ValueBox<Vec<PhlowObject>> {
    with_view(phlow_view, |phlow_view: &PhlowColumnedListView| {
        Ok(ValueBox::new(phlow_view.compute_items()))
    })
    .into_raw()
}

#[no_mangle]
pub extern "C" fn phlow_columned_list_get_columns_len(
    phlow_view: *mut ValueBox<Box<dyn PhlowView>>,
) -> usize {
    with_view(phlow_view, |phlow_view: &PhlowColumnedListView| {
        Ok(phlow_view.get_columns().len())
    })
    .or_log(0)
}

#[no_mangle]
pub extern "C" fn phlow_columned_list_get_column_title(
    phlow_view: *mut ValueBox<Box<dyn PhlowView>>,
    column_index: usize,
    column_title: *mut ValueBox<StringBox>,
) {
    with_view(phlow_view, |phlow_view: &PhlowColumnedListView| {
        column_title.with_mut_ok(|column_title| {
            if let Some(column) = phlow_view.get_columns().get(column_index) {
                column_title.set_string(column.get_title().to_string());
            }
        })
    })
    .log();
}

#[no_mangle]
pub extern "C" fn phlow_columned_list_view_compute_item_text_at(
    phlow_view: *mut ValueBox<Box<dyn PhlowView>>,
    items: *mut ValueBox<Vec<PhlowObject>>,
    row_index: usize,
    column_index: usize,
    item_text: *mut ValueBox<StringBox>,
) {
    with_view(phlow_view, |phlow_view: &PhlowColumnedListView| {
        items.with_ref(|items| {
            items
                .get(row_index)
                .ok_or_else(|| {
                    BoxerError::AnyError(format!("Item at {} does not exist", row_index).into())
                })
                .and_then(|item| {
                    item_text.with_mut(|item_text| {
                        phlow_view
                            .get_columns()
                            .get(column_index)
                            .ok_or_else(|| {
                                BoxerError::AnyError(
                                    format!("Column at {} does not exist", column_index).into(),
                                )
                            })
                            .and_then(|column| {
                                column
                                    .compute_cell_item(item)
                                    .ok_or_else(|| {
                                        BoxerError::AnyError(
                                            format!(
                                            "Could not compute cell item for row: {}, column: {}",
                                            row_index,
                                            column.get_title()
                                        )
                                            .into(),
                                        )
                                    })
                                    .map(|cell_item| {
                                        item_text.set_string(column.compute_cell_text(&cell_item))
                                    })
                            })
                    })
                })
        })
    })
    .log();
}

#[no_mangle]
pub extern "C" fn phlow_columned_list_view_compute_item_send_at(
    phlow_view: *mut ValueBox<Box<dyn PhlowView>>,
    items: *mut ValueBox<Vec<PhlowObject>>,
    index: usize,
) -> *mut ValueBox<PhlowObject> {
    with_view(phlow_view, |phlow_view: &PhlowColumnedListView| {
        items.with_ref(|items| {
            items
                .get(index)
                .ok_or_else(|| {
                    BoxerError::AnyError(format!("Item at {} does not exist", index).into())
                })
                .and_then(|item| {
                    phlow_view.compute_item_send(item).ok_or_else(|| {
                        BoxerError::AnyError(
                            format!("Couldn't determine an item to send at {}", index).into(),
                        )
                    })
                })
                .map(|item| ValueBox::new(item))
        })
    })
    .into_raw()
}
