use crate::{AnyValue, PhlowObject, PhlowViewMethod};

pub struct PhlowColumn {
    title: String,
    index: usize,
    text_computation: Box<dyn Fn(&AnyValue, &PhlowObject) -> String>,
}

impl PhlowColumn {
    pub fn new() -> Self {
        Self {
            title: "Column".to_string(),
            index: 0,
            text_computation: Box::new(|_item, object| object.to_string())
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn text<T: 'static>(
        mut self,
        item_text_block: impl Fn(&T, &PhlowObject) -> String + 'static,
    ) -> Self {
        self.text_computation =
            Box::new(
                move |each, each_object: &PhlowObject| match each.as_ref_safe::<T>() {
                    Some(each) => item_text_block(each, each_object),
                    None => "Error coercing item type".to_string(),
                },
            );
        self
    }
}

#[allow(unused)]
pub struct PhlowColumnView {
    object: PhlowObject,
    defining_method: PhlowViewMethod,
    title: String,
    priority: usize,
    columns: Vec<PhlowColumn>,
    items_computation: Box<dyn Fn(&PhlowObject) -> Vec<PhlowObject>>,
    send_computation: Box<dyn Fn(&AnyValue, &PhlowObject) -> Option<PhlowObject>>,
}

impl PhlowColumnView {
    pub fn new(object: PhlowObject, defining_method: PhlowViewMethod) -> Self {
        Self {
            object,
            defining_method,
            title: "".to_string(),
            priority: 10,
            columns: vec![],
            items_computation: Box::new(|_object| Default::default()),
            send_computation: Box::new(|_item, object| Some(object.clone())),
        }
    }

    pub fn column(mut self, column_block: impl FnOnce(PhlowColumn) -> PhlowColumn) -> Self {
        let mut new_column = PhlowColumn::new();
        // assign a provisional column index
        new_column.index = self.columns.len();

        let mut new_column = (column_block)(new_column);
        // assign the index again in case user created a completely new column from scratch
        new_column.index = self.columns.len();

        self.columns.push(new_column);
        self
    }
}
