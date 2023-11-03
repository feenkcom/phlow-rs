use std::ffi::c_void;
use std::fmt::Debug;
use std::slice::Iter;
use std::vec::IntoIter;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

#[derive(Default)]
pub struct AnyMap(HashMap<String, AnyValue>);

#[allow(unused)]
impl AnyMap {
    pub fn push(&mut self, key: impl Into<String>, value: AnyValue) {
        self.0.insert(key.into(), value);
    }

    pub fn get<T: 'static>(&self, key: impl Into<String>) -> Option<&T> {
        match self.0.get(&key.into()) {
            Some(item) => item.as_ref_safe(),
            None => None,
        }
    }
}

#[allow(dead_code)]
#[allow(unused)]
#[derive(Debug)]
pub struct AnyObject {
    object: Box<dyn Any>,
    object_ptr: *const c_void,
}

#[allow(unused)]
impl AnyObject {
    pub fn new<T: Any>(object: T) -> Self {
        let object_box = Box::new(object);
        let object_ptr = object_box.as_ref() as *const T as *const c_void;

        Self {
            object: object_box,
            object_ptr,
        }
    }

    pub fn as_ref<T: Any>(&self) -> &T {
        self.as_ref_safe().unwrap()
    }

    pub fn as_ptr(&self) -> *const c_void {
        self.object_ptr
    }

    pub fn as_ref_safe<T: Any>(&self) -> Option<&T> {
        self.object.downcast_ref()
    }

    pub fn as_mut_safe<T: Any>(&mut self) -> Option<&mut T> {
        self.object.downcast_mut()
    }

    pub fn take_value<T: Any>(self) -> Option<T> {
        match self.object.downcast::<T>() {
            Ok(value) => Some(*value),
            Err(value) => None,
        }
    }

    pub fn clone_value<T: Any + Clone>(&self) -> Option<T> {
        self.object.downcast_ref::<T>().cloned()
    }
}

#[derive(Debug)]
pub struct AnyReference {
    reference_ptr: *const c_void,
    type_id: TypeId,
}

impl AnyReference {
    pub fn new<T: 'static + ?Sized>(reference: &T) -> Self {
        Self {
            reference_ptr: reference as *const T as *const c_void,
            type_id: TypeId::of::<T>(),
        }
    }

    pub fn as_ref<T: 'static>(&self) -> &T {
        self.as_ref_safe().unwrap()
    }

    pub fn as_ref_safe<T: 'static>(&self) -> Option<&T> {
        if self.type_id == TypeId::of::<T>() {
            Some(unsafe { &*(self.reference_ptr as *const T) })
        } else {
            None
        }
    }

    pub fn as_mut_safe<T: Any>(&mut self) -> Option<&mut T> {
        if self.type_id == TypeId::of::<T>() {
            Some(unsafe { &mut *(self.reference_ptr as *mut T) })
        } else {
            None
        }
    }

    pub fn as_ptr(&self) -> *const c_void {
        self.reference_ptr
    }

    pub fn clone_value<T: Any + Clone>(&self) -> Option<T> {
        self.as_ref_safe().cloned()
    }
}

#[derive(Debug)]
pub enum AnyValue {
    Object(AnyObject),
    Reference(AnyReference),
    None,
}

impl AnyValue {
    pub fn object<T: 'static>(object: T) -> Self {
        Self::Object(AnyObject::new(object))
    }

    pub fn reference<T: 'static>(reference: &T) -> Self {
        Self::Reference(AnyReference::new(reference))
    }

    pub fn as_ref_safe<T: 'static>(&self) -> Option<&T> {
        match self {
            AnyValue::Object(object) => object.as_ref_safe(),
            AnyValue::Reference(reference) => reference.as_ref_safe(),
            AnyValue::None => None,
        }
    }

    pub fn as_mut_safe<T: 'static>(&mut self) -> Option<&mut T> {
        match self {
            AnyValue::Object(object) => object.as_mut_safe(),
            AnyValue::Reference(reference) => reference.as_mut_safe(),
            AnyValue::None => None,
        }
    }

    pub fn take_value<T: Any>(self) -> Option<T> {
        match self {
            AnyValue::Object(value) => value.take_value(),
            AnyValue::Reference(_) => None,
            AnyValue::None => None,
        }
    }

    pub fn clone_value<T: Any + Clone>(&self) -> Option<T> {
        match self {
            AnyValue::Object(value) => value.clone_value(),
            AnyValue::Reference(reference) => reference.clone_value(),
            AnyValue::None => None,
        }
    }

    pub fn as_ptr(&self) -> *const c_void {
        match self {
            AnyValue::Object(object) => object.as_ptr(),
            AnyValue::Reference(reference) => reference.as_ptr(),
            AnyValue::None => std::ptr::null(),
        }
    }

    pub fn has_value(&self) -> bool {
        match self {
            AnyValue::Object(_) => true,
            AnyValue::Reference(_) => true,
            AnyValue::None => false,
        }
    }
}

unsafe impl Sync for AnyValue {}
unsafe impl Send for AnyValue {}

#[derive(Default)]
pub struct AnyVec(Vec<AnyValue>);

#[allow(unused)]
impl AnyVec {
    pub fn push<T: 'static>(&mut self, value: T) {
        self.0.push(AnyValue::object(value));
    }

    pub fn get(&self, index: usize) -> Option<&AnyValue> {
        self.0.get(index)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> Iter<'_, AnyValue> {
        self.0.iter()
    }

    pub fn into_iter(self) -> IntoIter<AnyValue> {
        self.0.into_iter()
    }
}

pub fn type_id_of_val<T: ?Sized + 'static>(_val: &T) -> TypeId {
    TypeId::of::<T>()
}
