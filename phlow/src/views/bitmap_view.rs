use std::any::Any;
use std::fmt::{Debug, Display, Formatter};

use crate::{PhlowObject, PhlowView, PhlowViewMethod, TypedPhlowObject, TypedPhlowObjectMut};

#[allow(unused)]
pub struct PhlowBitmapView {
    object: PhlowObject,
    defining_method: PhlowViewMethod,
    title: String,
    priority: usize,
    bitmap_computation: Box<dyn Fn(&PhlowObject) -> PhlowBitmap>,
}

impl PhlowBitmapView {
    pub fn new(object: PhlowObject, defining_method: PhlowViewMethod) -> Self {
        Self {
            object,
            defining_method,
            title: "".to_string(),
            priority: 10,
            bitmap_computation: Box::new(|_object| PhlowBitmap::default()),
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

    pub fn bitmap<T: 'static>(
        mut self,
        bitmap_block: impl Fn(TypedPhlowObject<T>) -> PhlowBitmap + 'static,
    ) -> Self {
        self.bitmap_computation = Box::new(move |each_object| match each_object.value_ref::<T>() {
            Some(each_reference) => {
                bitmap_block(TypedPhlowObject::new(each_object, &each_reference))
            }
            None => PhlowBitmap::default(),
        });
        self
    }

    pub fn bitmap_mut<T: 'static>(
        mut self,
        bitmap_block: impl Fn(TypedPhlowObjectMut<T>) -> PhlowBitmap + 'static,
    ) -> Self {
        self.bitmap_computation = Box::new(move |each_object| match each_object.value_mut::<T>() {
            Some(mut each_reference) => {
                bitmap_block(TypedPhlowObjectMut::new(each_object, &mut each_reference))
            }
            None => PhlowBitmap::default(),
        });
        self
    }

    pub fn compute_bitmap(&self) -> PhlowBitmap {
        (self.bitmap_computation)(&self.object)
    }
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "view-specification", derive(serde::Serialize))]
pub struct PhlowBitmap {
    pixels: Vec<u8>,
    width: i32,
    height: i32,
    stride: i32,
    format: PixelFormat,
}

impl PhlowBitmap {
    pub fn new(
        pixels: impl ToOwned<Owned = Vec<u8>>,
        width: i32,
        height: i32,
        stride: i32,
        format: PixelFormat,
    ) -> Self {
        Self {
            pixels: pixels.to_owned(),
            width,
            height,
            stride,
            format,
        }
    }

    pub fn rgba8(pixels: impl ToOwned<Owned = Vec<u8>>, width: i32, height: i32) -> Self {
        Self::new(pixels, width, height, width, PixelFormat::RGBA8888)
    }

    pub fn bgra8(pixels: impl ToOwned<Owned = Vec<u8>>, width: i32, height: i32) -> Self {
        Self::new(pixels, width, height, width, PixelFormat::BGRA8888)
    }

    pub fn pixels(&self) -> &[u8] {
        self.pixels.as_slice()
    }

    pub fn stride(&self) -> i32 {
        self.stride
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "view-specification", derive(serde::Serialize))]
pub enum PixelFormat {
    RGBA8888,
    BGRA8888,
}

impl Default for PixelFormat {
    fn default() -> Self {
        Self::RGBA8888
    }
}

impl Debug for PhlowBitmapView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PhlowBitmapView").finish()
    }
}

impl Display for PhlowBitmapView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.title.as_str())?;
        writeln!(f, "{}", "---------------------",)?;

        let bitmap = self.compute_bitmap();
        writeln!(
            f,
            "Bitmap: {}x{}; format: {:?}",
            bitmap.width, bitmap.height, bitmap.format
        )?;

        Ok(())
    }
}

impl PhlowView for PhlowBitmapView {
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
        "bitmap_view"
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
    use serde::Serialize;

    use crate::{
        AsPhlowViewSpecification, PhlowViewSpecification, PhlowViewSpecificationDataTransport,
        PhlowViewSpecificationListingItem,
    };

    use super::*;

    #[derive(Debug, Clone, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PhlowBitmapViewSpecification {
        title: String,
        priority: usize,
        data_transport: PhlowViewSpecificationDataTransport,
        bitmap: PhlowBitmap,
        method_selector: String,
    }

    #[typetag::serialize(name = "GtPhlowBitmapViewSpecification")]
    impl PhlowViewSpecification for PhlowBitmapViewSpecification {
        fn retrieve_items(&self) -> Vec<Box<dyn PhlowViewSpecificationListingItem>> {
            vec![]
        }

        fn retrieve_sent_item(&self, item: &PhlowObject) -> PhlowObject {
            item.clone()
        }
    }

    impl AsPhlowViewSpecification for PhlowBitmapView {
        fn create_view_specification(&self) -> Option<Box<dyn PhlowViewSpecification>> {
            Some(Box::new(PhlowBitmapViewSpecification {
                title: self.get_title().to_string(),
                priority: self.get_priority(),
                data_transport: PhlowViewSpecificationDataTransport::Included,
                bitmap: self.compute_bitmap(),
                method_selector: self.get_defining_method().full_method_name.clone(),
            }))
        }
    }
}
