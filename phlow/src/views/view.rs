use futures_util::FutureExt;
use std::any::Any;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::future::ready;
use std::pin::Pin;
use std::sync::Arc;

use crate::{
    AsyncComputation, AsyncComputationFuture, Phlow, PhlowBitmapView, PhlowColumnedListView,
    PhlowListView, PhlowObject, PhlowTextView, PhlowViewMethod, SyncComputation,
    SyncMutComputation, TypedPhlowObject, TypedPhlowObjectMut,
};

pub trait PhlowView: Debug + Display + Any {
    fn get_title(&self) -> &str;
    fn get_priority(&self) -> usize;
    fn get_view_type(&self) -> &str;
    fn get_defining_method(&self) -> &PhlowViewMethod;
    fn view_type() -> &'static str
    where
        Self: Sized;
    fn object(&self) -> &PhlowObject;
    fn list(&self) -> PhlowListView {
        PhlowListView::new(self.object().clone(), self.get_defining_method().clone())
    }
    fn columned_list(&self) -> PhlowColumnedListView {
        PhlowColumnedListView::new(self.object().clone(), self.get_defining_method().clone())
    }
    fn text(&self) -> PhlowTextView {
        PhlowTextView::new(self.object().clone(), self.get_defining_method().clone())
    }
    fn bitmap(&self) -> PhlowBitmapView {
        PhlowBitmapView::new(self.object().clone(), self.get_defining_method().clone())
    }
    fn as_any(&self) -> &dyn Any;
    fn to_any(self: Box<Self>) -> Box<dyn Any>;
    #[cfg(feature = "view-specification")]
    fn as_view_specification_builder(&self) -> &dyn crate::AsPhlowViewSpecification;
    #[cfg(feature = "view-specification")]
    fn as_view_specification(&self) -> Option<Box<dyn crate::PhlowViewSpecification>> {
        self.as_view_specification_builder()
            .create_view_specification()
    }
}

#[derive(Debug)]
pub struct PhlowViewContext {}

#[derive(Debug)]
pub struct PhlowProtoView {
    object: PhlowObject,
    defining_method: PhlowViewMethod,
}

impl PhlowProtoView {
    pub fn new(object: PhlowObject, defining_method: PhlowViewMethod) -> Self {
        Self {
            object,
            defining_method,
        }
    }
}

impl Display for PhlowProtoView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ProtoView")
    }
}

impl PhlowView for PhlowProtoView {
    fn get_title(&self) -> &str {
        "Untitled"
    }

    fn get_priority(&self) -> usize {
        10
    }

    fn get_view_type(&self) -> &str {
        Self::view_type()
    }

    fn get_defining_method(&self) -> &PhlowViewMethod {
        &self.defining_method
    }

    fn view_type() -> &'static str {
        "proto_view"
    }

    fn object(&self) -> &PhlowObject {
        &self.object
    }

    fn as_any(&self) -> &dyn Any {
        todo!()
    }

    fn to_any(self: Box<Self>) -> Box<dyn Any> {
        todo!()
    }

    #[cfg(feature = "view-specification")]
    fn as_view_specification_builder(&self) -> &dyn crate::AsPhlowViewSpecification {
        self
    }
}

pub fn downcast_view_ref<T: PhlowView>(
    phlow_view: &Box<dyn PhlowView>,
) -> Result<&T, Box<dyn Error>> {
    if phlow_view.get_view_type() == T::view_type() {
        let view_any = phlow_view.as_any();
        let view = unsafe { &*(view_any as *const dyn Any as *const T) };
        Ok(view)
    } else {
        Err(Into::<Box<dyn Error>>::into(format!(
            "Failed to downcast {} to {}",
            phlow_view.get_view_type(),
            T::view_type()
        )))
    }
}

/// Represents a computation that can be either sync or async
#[derive(Clone)]
pub enum Computation<Return> {
    Sync(Arc<dyn Fn(&PhlowObject) -> Option<Return> + Send + Sync>),
    Async(
        Arc<
            dyn Fn(
                    &PhlowObject,
                )
                    -> Option<Pin<Box<dyn AsyncComputationFuture<Return, Output = Return>>>>
                + Send
                + Sync,
        >,
    ),
}
impl<Return: Send + Sync + 'static> Computation<Return> {
    pub fn new_sync<T: 'static>(items_block: impl SyncComputation<T, Return>) -> Self {
        Self::Sync(Arc::new(move |object: &PhlowObject| {
            object
                .value_ref::<T>()
                .map(|reference| items_block(TypedPhlowObject::new(object, &reference)))
        }))
    }

    pub fn new_sync_mut<T: 'static>(items_block: impl SyncMutComputation<T, Return>) -> Self {
        Self::Sync(Arc::new(move |object: &PhlowObject| {
            object
                .value_mut::<T>()
                .map(|mut reference| items_block(TypedPhlowObjectMut::new(object, &mut reference)))
        }))
    }

    pub fn new_async<T: 'static, F: AsyncComputationFuture<Return>>(
        items_block: impl AsyncComputation<T, Return, F>,
    ) -> Self {
        Self::Async(Arc::new(move |object: &PhlowObject| {
            object.value_ref::<T>().map(|reference| {
                let value = Box::pin(items_block(TypedPhlowObject::new(object, &reference)));
                value as Pin<Box<(dyn AsyncComputationFuture<Return, Output = Return> + 'static)>>
            })
        }))
    }

    pub async fn value(&self, object: &PhlowObject) -> Option<Return> {
        match self {
            Self::Sync(computation) => ready((computation)(object)).await,
            Self::Async(computation) => {
                let value = (computation)(object);
                match value {
                    None => None,
                    Some(future) => future.map(|value| Some(value)).await,
                }
            }
        }
    }

    pub async fn value_or_else(&self, object: &PhlowObject, f: impl Fn() -> Return) -> Return {
        match self {
            Self::Sync(computation) => {
                let value = (computation)(object);
                ready(value.unwrap_or_else(f)).await
            }
            Self::Async(computation) => {
                let value = (computation)(object);
                value.unwrap_or_else(|| Box::pin(ready(f()))).await
            }
        }
    }

    pub fn value_block_on(&self, object: &PhlowObject) -> Option<Return> {
        match self {
            Self::Sync(computation) => (computation)(object),
            Self::Async(computation) => {
                (computation)(object).map(|future| futures_executor::block_on(future))
            }
        }
    }

    pub fn is_async(&self) -> bool {
        match self {
            Self::Sync(_) => false,
            Self::Async(_) => true,
        }
    }
}

pub type ItemsComputation = Computation<Vec<PhlowObject>>;
pub type TextComputation = Computation<String>;
pub type SendComputation = Computation<PhlowObject>;
pub type ItemComputation = Computation<PhlowObject>;

impl Default for ItemsComputation {
    fn default() -> Self {
        Self::Sync(Arc::new(|_| Some(vec![])))
    }
}

impl Default for TextComputation {
    fn default() -> Self {
        Self::Sync(Arc::new(|object| Some(object.to_string())))
    }
}
impl Default for Computation<PhlowObject> {
    fn default() -> Self {
        Self::Sync(Arc::new(|object| Some(object.clone())))
    }
}

pub mod types {
    use std::future::Future;

    use crate::{TypedPhlowObject, TypedPhlowObjectMut};

    pub trait SyncComputation<T, R>: Fn(TypedPhlowObject<T>) -> R + Send + Sync + 'static {}
    impl<T, R, O: Fn(TypedPhlowObject<T>) -> R + Send + Sync + 'static> SyncComputation<T, R> for O {}

    pub trait SyncMutComputation<T, R>:
        Fn(TypedPhlowObjectMut<T>) -> R + Send + Sync + 'static
    {
    }

    impl<T, R, O: Fn(TypedPhlowObjectMut<T>) -> R + Send + Sync + 'static> SyncMutComputation<T, R>
        for O
    {
    }

    pub trait AsyncComputationFuture<T>: Future<Output = T> + Send + Sync + 'static {}

    impl<T, O: Future<Output = T> + Send + Sync + 'static> AsyncComputationFuture<T> for O {}

    pub trait AsyncComputation<T, R, F>:
        Fn(TypedPhlowObject<T>) -> F + Send + Sync + 'static
    where
        F: AsyncComputationFuture<R>,
    {
    }
}
