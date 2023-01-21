#[macro_use]
extern crate phlow;

use phlow::AsPhlowObject;

import_extensions!();

#[test]
pub fn phlow_object() {
    let value = 42;
    let object = phlow!(value);

    assert!(object.is_phlow_object());
    assert_eq!(object.value_type_name(), "i32");
}

#[test]
pub fn phlow_reference() {
    let value = 42;
    // this works because i32 implements Clone
    let object = phlow_ref!(&value);

    assert!(object.is_phlow_object());
    assert_eq!(object.value_type_name(), "i32");
}

#[test]
pub fn phlow_reference_with_parent() {
    let parent = phlow!(vec![0, 1, 2]);

    let parent_ref = parent.value_ref::<Vec<i32>>().unwrap();
    let child = parent_ref.get(0).unwrap();

    // works when we pass parent by value
    let object = phlow_ref!(child, parent);
    assert!(object.is_phlow_object());
    assert_eq!(object.value_type_name(), "i32");

    // also works when passing a reference to the parent
    let object = phlow_ref!(child, &parent);
    assert!(object.is_phlow_object());
    assert_eq!(object.value_type_name(), "i32");
}

#[test]
pub fn phlow_object_with_generic() {
    let value: Vec<i32> = vec![0, 1, 2];
    let object = phlow!(value, <i32>);

    assert!(object.is_phlow_object());
    assert_eq!(object.value_type_name(), "alloc::vec::Vec<i32>");

    let generic_types = object.generic_phlow_types();
    assert_eq!(generic_types.len(), 1);
    assert_eq!(generic_types[0].type_name(), "i32");
}

#[test]
pub fn phlow_phlow_object() {
    let value = 42;
    let object = phlow!(phlow!(value));

    assert!(object.is_phlow_object());
    assert_eq!(object.value_type_name(), "i32");
}

#[test]
pub fn phlow_reference_to_phlow_object() {
    let value = 42;

    let object_1 = phlow!(value);
    let object_1_ref = &object_1;
    let object_2 = phlow!(object_1_ref.clone());

    assert!(object_1.is_phlow_object());
    assert_eq!(object_1.value_type_name(), "i32");
    assert!(object_2.is_phlow_object());
    assert_eq!(object_2.value_type_name(), "i32");
    assert_eq!(object_1.value_ptr(), object_2.value_ptr());
}
