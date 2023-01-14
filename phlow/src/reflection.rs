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
    pub fn new<T: 'static>(object: T) -> Self {
        let object_box = Box::new(object);
        let object_ptr = object_box.as_ref() as *const T as *const c_void;

        Self {
            object: object_box,
            object_ptr,
        }
    }

    pub fn as_ref<T: 'static>(&self) -> &T {
        self.as_ref_safe().unwrap()
    }

    pub fn as_ptr(&self) -> *const c_void {
        self.object_ptr
    }

    pub fn as_ref_safe<T: 'static>(&self) -> Option<&T> {
        self.object.downcast_ref()
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

    pub fn as_ptr(&self) -> *const c_void {
        self.reference_ptr
    }
}

#[derive(Debug)]
pub enum AnyValue {
    Object(AnyObject),
    Reference(AnyReference),
}

impl AnyValue {
    pub fn object<T: 'static>(object: T) -> Self {
        Self::Object(AnyObject::new(object))
    }

    pub fn reference<T: 'static>(reference: &T) -> Self {
        Self::Reference(AnyReference::new(reference))
    }

    pub fn as_ref<T: 'static>(&self) -> &T {
        match self {
            AnyValue::Object(object) => object.as_ref(),
            AnyValue::Reference(reference) => reference.as_ref(),
        }
    }

    pub fn as_ref_safe<T: 'static>(&self) -> Option<&T> {
        match self {
            AnyValue::Object(object) => object.as_ref_safe(),
            AnyValue::Reference(reference) => reference.as_ref_safe(),
        }
    }

    pub fn as_ptr(&self) -> *const c_void {
        match self {
            AnyValue::Object(object) => object.as_ptr(),
            AnyValue::Reference(reference) => reference.as_ptr(),
        }
    }
}

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

#[cfg(test)]
mod tests {
    use std::any::type_name;
    use std::fmt::format;
    use std::mem::transmute;

    use super::*;

    pub fn type_name_of_val<T: ?Sized>(_val: &T) -> &'static str {
        type_name::<T>()
    }

    #[test]
    pub fn talented_object() {
        let i = 20;
        let type_name = type_name_of_val(&i);

        let library = open_self();
        println!("{:?}", library);
        println!("{:?}", type_name);

        let func_name = format!("phlow_types_{}", type_name);

        let func: Result<Symbol<unsafe extern "C" fn() -> ()>, libloading::Error> =
            unsafe { library.get(func_name.as_bytes()) };
        println!("{:?}", func);
    }
}
