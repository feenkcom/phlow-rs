use std::any::Any;
use std::fmt::{Debug, Display, Formatter};

use futures_util::{stream, FutureExt, Stream, StreamExt};

use crate::{
    AsyncComputation, ItemsComputation, PhlowObject, PhlowView, PhlowViewMethod, SendComputation,
    SyncComputation, SyncMutComputation, TextComputation,
};

#[allow(unused)]
#[derive(Clone)]
pub struct PhlowListView {
    object: PhlowObject,
    defining_method: PhlowViewMethod,
    title: String,
    priority: usize,
    items_computation: ItemsComputation,
    item_text_computation: TextComputation,
    send_computation: SendComputation,
}

impl PhlowListView {
    pub fn new(object: PhlowObject, defining_method: PhlowViewMethod) -> Self {
        Self {
            object,
            defining_method,
            title: "".to_string(),
            priority: 10,
            items_computation: Default::default(),
            item_text_computation: Default::default(),
            send_computation: Default::default(),
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
        items_block: impl SyncComputation<T, Vec<PhlowObject>>,
    ) -> Self {
        self.items_computation = ItemsComputation::new_sync(items_block);
        self
    }

    pub fn items_mut<T: 'static>(
        mut self,
        items_block: impl SyncMutComputation<T, Vec<PhlowObject>>,
    ) -> Self {
        self.items_computation = ItemsComputation::new_sync_mut(items_block);
        self
    }

    pub fn async_items<T: 'static>(
        mut self,
        items_block: impl AsyncComputation<T, Vec<PhlowObject>>,
    ) -> Self {
        self.items_computation = ItemsComputation::new_async(items_block);
        self
    }

    pub fn item_text<T: 'static>(
        mut self,
        item_text_block: impl SyncComputation<T, String>,
    ) -> Self {
        self.item_text_computation = TextComputation::new_sync(item_text_block);
        self
    }

    pub fn send<T: 'static>(
        mut self,
        item_send_block: impl SyncComputation<T, PhlowObject>,
    ) -> Self {
        self.send_computation = SendComputation::new_sync(item_send_block);
        self
    }

    pub fn compute_items(&self) -> impl Stream<Item = PhlowObject> + '_ {
        self.items_computation
            .value_or_else(&self.object, || vec![])
            .into_stream()
            .map(|items| stream::iter(items))
            .flatten()
    }

    pub fn compute_items_sync(&self) -> Vec<PhlowObject> {
        self.items_computation
            .value_block_on(&self.object)
            .unwrap_or_default()
    }

    pub async fn compute_item_to_send(&self, item: &PhlowObject) -> Option<PhlowObject> {
        self.send_computation.value(item).await
    }

    pub fn compute_item_to_send_sync(&self, item: &PhlowObject) -> Option<PhlowObject> {
        self.send_computation.value_block_on(item)
    }

    pub fn compute_item_text_sync(&self, item: &PhlowObject) -> String {
        self.item_text_computation
            .value_block_on(item)
            .unwrap_or_else(|| "Error coercing item type".to_string())
    }

    pub async fn compute_item_text(&self, item: &PhlowObject) -> String {
        self.item_text_computation
            .value_or_else(item, || "Error coercing item type".to_string())
            .await
    }
}

impl Debug for PhlowListView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PhlowListView").finish()
    }
}

impl Display for PhlowListView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.title.as_str())?;
        writeln!(f, "{}", "---------------------",)?;
        writeln!(f, "{0: >3} | {1}", "i", "item",)?;

        let items = self.compute_items_sync();

        for (index, item) in items.into_iter().enumerate() {
            writeln!(f, "{0:>3} | {1}", index, self.compute_item_text_sync(&item))?;
        }

        Ok(())
    }
}

impl PhlowView for PhlowListView {
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

    fn view_type() -> &'static str {
        "list_view"
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

const _: () = {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    // RFC 2056
    fn assert_all() {
        assert_send::<PhlowListView>();
    }
};

#[cfg(feature = "view-specification")]
mod specification {
    use serde::Serialize;

    use crate::views::view_specification::PhlowViewSpecificationTextualItemValue;
    use crate::{
        AsPhlowViewSpecification, PhlowViewSpecification, PhlowViewSpecificationDataTransport,
        PhlowViewSpecificationListingItem,
    };

    use super::*;

    #[derive(Debug, Clone, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PhlowListViewSpecification {
        title: String,
        priority: usize,
        data_transport: PhlowViewSpecificationDataTransport,
        method_selector: String,
        #[serde(skip)]
        phlow_view: PhlowListView,
    }

    #[typetag::serialize(name = "GtPhlowListViewSpecification")]
    #[async_trait::async_trait]
    impl PhlowViewSpecification for PhlowListViewSpecification {
        async fn retrieve_items(&self) -> Vec<Box<dyn PhlowViewSpecificationListingItem>> {
            self.phlow_view
                .compute_items()
                .then(|each| async move {
                    Box::new(PhlowViewSpecificationTextualItemValue {
                        phlow_object: each.clone(),
                        item_text: self.phlow_view.compute_item_text(&each).await,
                    }) as Box<dyn PhlowViewSpecificationListingItem>
                })
                .collect()
                .await
        }

        async fn retrieve_sent_item(&self, item: &PhlowObject) -> Option<PhlowObject> {
            self.phlow_view.compute_item_to_send(item).await
        }
    }

    impl AsPhlowViewSpecification for PhlowListView {
        fn create_view_specification(&self) -> Option<Box<dyn PhlowViewSpecification>> {
            Some(Box::new(PhlowListViewSpecification {
                title: self.get_title().to_string(),
                priority: self.get_priority(),
                data_transport: PhlowViewSpecificationDataTransport::Lazy,
                method_selector: self.get_defining_method().full_method_name.clone(),
                phlow_view: self.clone(),
            }))
        }
    }
}
