use std::any::Any;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;

use crate::{PhlowObject, PhlowView, PhlowViewMethod, TypedPhlowObject};

#[derive(Clone)]
pub struct PhlowColumn {
    title: String,
    index: usize,
    item_computation: Arc<dyn Fn(&PhlowObject) -> Option<PhlowObject>>,
    text_computation: Arc<dyn Fn(&PhlowObject) -> String>,
}

impl PhlowColumn {
    pub fn new() -> Self {
        Self {
            title: "Column".to_string(),
            index: 0,
            item_computation: Arc::new(|object| Some(object.clone())),
            text_computation: Arc::new(|object| object.to_string()),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn item<T: 'static>(
        mut self,
        item_computation: impl Fn(TypedPhlowObject<T>) -> PhlowObject + 'static,
    ) -> Self {
        self.item_computation = Arc::new(move |each_object| {
            each_object.value_ref::<T>().map(|each_reference| {
                item_computation(TypedPhlowObject::new(each_object, &each_reference))
            })
        });
        self
    }

    pub fn text<T: 'static>(
        mut self,
        text_block: impl Fn(TypedPhlowObject<T>) -> String + 'static,
    ) -> Self {
        self.text_computation =
            Arc::new(
                move |each_object: &PhlowObject| match each_object.value_ref::<T>() {
                    Some(each_reference) => {
                        text_block(TypedPhlowObject::new(each_object, &each_reference))
                    }
                    None => "Error coercing item type".to_string(),
                },
            );
        self
    }

    pub fn get_title(&self) -> &str {
        self.title.as_str()
    }

    pub fn compute_cell_item(&self, row_object: &PhlowObject) -> Option<PhlowObject> {
        (self.item_computation)(row_object)
    }

    pub fn compute_cell_text(&self, cell_object: &PhlowObject) -> String {
        (self.text_computation)(cell_object)
    }
}

#[allow(unused)]
#[derive(Clone)]
pub struct PhlowColumnedListView {
    object: PhlowObject,
    defining_method: PhlowViewMethod,
    title: String,
    priority: usize,
    columns: Vec<PhlowColumn>,
    items_computation: Arc<dyn Fn(&PhlowObject) -> Vec<PhlowObject>>,
    send_computation: Arc<dyn Fn(&PhlowObject) -> Option<PhlowObject>>,
}

impl PhlowColumnedListView {
    pub fn new(object: PhlowObject, defining_method: PhlowViewMethod) -> Self {
        Self {
            object,
            defining_method,
            title: "".to_string(),
            priority: 10,
            columns: vec![],
            items_computation: Arc::new(|_object| Default::default()),
            send_computation: Arc::new(|object| Some(object.clone())),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn priority(mut self, priority: usize) -> Self {
        self.priority = priority;
        self
    }

    pub fn items<T: 'static>(
        mut self,
        items_block: impl Fn(TypedPhlowObject<T>) -> Vec<PhlowObject> + 'static,
    ) -> Self {
        self.items_computation = Arc::new(move |object: &PhlowObject| {
            // the type may differ when passing over ffi boundary...
            if let Some(reference) = object.value_ref::<T>() {
                items_block(TypedPhlowObject::new(object, &reference))
            } else {
                vec![]
            }
        });
        self
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

    pub fn column_item<T: 'static>(
        self,
        title: impl Into<String>,
        item_computation: impl Fn(TypedPhlowObject<T>) -> PhlowObject + 'static,
    ) -> Self {
        self.column(|column| column.title(title).item(item_computation))
    }

    pub fn send<T: 'static>(
        mut self,
        item_send_block: impl Fn(TypedPhlowObject<T>) -> PhlowObject + 'static,
    ) -> Self {
        self.send_computation = Arc::new(move |object| {
            object
                .value_ref::<T>()
                .map(|item| item_send_block(TypedPhlowObject::new(object, &item)))
        });
        self
    }

    pub fn compute_items(&self) -> Vec<PhlowObject> {
        (self.items_computation)(&self.object)
    }
    pub fn compute_item_send(&self, item: &PhlowObject) -> PhlowObject {
        ((self.send_computation)(item)).unwrap_or_else(|| item.clone())
    }
    pub fn get_columns(&self) -> &[PhlowColumn] {
        self.columns.as_slice()
    }
}

impl Debug for PhlowColumnedListView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PhlowColumnListView").finish()
    }
}

impl Display for PhlowColumnedListView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.get_title())?;
        writeln!(f, "{}", "---------------------",)?;

        for column in &self.columns {
            if column.index > 0 {
                write!(f, " | ")?;
            }
            write!(f, "{0: <10}", column.title.as_str())?;
        }
        writeln!(f, "")?;

        let items = self.compute_items();
        for row_item in items {
            for column in &self.columns {
                let cell_item = column.compute_cell_item(&row_item);
                if column.index > 0 {
                    write!(f, " | ")?;
                }

                let cell_text = cell_item
                    .as_ref()
                    .map(|valid_cell_item| column.compute_cell_text(valid_cell_item))
                    .unwrap_or_else(|| "Error coercing item type".to_string());

                write!(f, "{0: <10}", cell_text)?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

impl PhlowView for PhlowColumnedListView {
    fn get_title(&self) -> &str {
        self.title.as_str()
    }

    fn get_priority(&self) -> usize {
        self.priority
    }

    fn get_view_type(&self) -> &str {
        Self::view_type()
    }

    fn get_defining_method(&self) -> &PhlowViewMethod {
        &self.defining_method
    }

    fn view_type() -> &'static str
    where
        Self: Sized,
    {
        "columned_list_view"
    }

    fn object(&self) -> &PhlowObject {
        &self.object
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    #[cfg(feature = "view-specification")]
    fn as_view_specification_builder(&self) -> &dyn crate::AsPhlowViewSpecification {
        self
    }
}

#[cfg(feature = "view-specification")]
mod specification {
    use super::*;

    use crate::views::view_specification::{
        PhlowViewSpecificationItemValue, PhlowViewSpecificationRowValue,
    };
    use crate::{
        AsPhlowViewSpecification, PhlowViewSpecification, PhlowViewSpecificationDataTransport,
        PhlowViewSpecificationListingItem, PhlowViewSpecificationListingType,
    };
    use serde::Serialize;

    #[derive(Debug, Clone, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PhlowColumnedListViewSpecification {
        title: String,
        priority: usize,
        data_transport: PhlowViewSpecificationDataTransport,
        method_selector: String,
        column_specifications: Vec<PhlowColumnSpecification>,
        #[serde(skip)]
        phlow_view: PhlowColumnedListView,
    }

    #[derive(Debug, Clone, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PhlowColumnSpecification {
        title: String,
        cell_width: Option<f32>,
        spawns_objects: bool,
        r#type: PhlowViewSpecificationListingType,
        properties: Vec<String>,
    }

    #[typetag::serialize(name = "GtPhlowColumnedListViewSpecification")]
    impl PhlowViewSpecification for PhlowColumnedListViewSpecification {
        fn retrieve_items(&self) -> Vec<Box<dyn PhlowViewSpecificationListingItem>> {
            self.phlow_view
                .compute_items()
                .into_iter()
                .map(|each| {
                    Box::new(PhlowViewSpecificationRowValue {
                        phlow_object: each.clone(),
                        column_values: self
                            .phlow_view
                            .columns
                            .iter()
                            .map(|column| {
                                column
                                    .compute_cell_item(&each)
                                    .map(|cell_object| column.compute_cell_text(&cell_object))
                                    .unwrap_or_else(|| "".to_string())
                            })
                            .map(|cell_text| PhlowViewSpecificationItemValue {
                                phlow_object: each.clone(),
                                item_text: cell_text,
                            })
                            .collect(),
                    }) as Box<dyn PhlowViewSpecificationListingItem>
                })
                .collect()
        }

        fn retrieve_sent_item(&self, item: &PhlowObject) -> PhlowObject {
            self.phlow_view.compute_item_send(item)
        }
    }

    impl AsPhlowViewSpecification for PhlowColumnedListView {
        fn create_view_specification(&self) -> Option<Box<dyn PhlowViewSpecification>> {
            Some(Box::new(PhlowColumnedListViewSpecification {
                title: self.get_title().to_string(),
                priority: self.get_priority(),
                data_transport: PhlowViewSpecificationDataTransport::Lazy,
                method_selector: self.get_defining_method().full_method_name.clone(),
                column_specifications: self
                    .columns
                    .iter()
                    .map(|column| PhlowColumnSpecification {
                        title: column.get_title().to_string(),
                        cell_width: None,
                        spawns_objects: false,
                        r#type: PhlowViewSpecificationListingType::Text,
                        properties: vec![],
                    })
                    .collect(),
                phlow_view: self.clone(),
            }))
        }
    }
}
