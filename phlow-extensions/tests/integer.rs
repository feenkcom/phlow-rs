#[macro_use]
extern crate phlow;
extern crate phlow_extensions;

use phlow_extensions::CoreExtensions;

import_extensions!(CoreExtensions);

fn assert_has_extensions<T: 'static>(value: T) {
    let object = phlow!(value);
    let views = object.phlow_views();
    assert!(views.len() > 0);
}

#[test]
pub fn test_u8() {
    assert_has_extensions(42u8);
}

#[test]
pub fn test_i8() {
    assert_has_extensions(42i8);
}

#[test]
pub fn test_u16() {
    assert_has_extensions(42u16);
}

#[test]
pub fn test_i16() {
    assert_has_extensions(42i16);
}

#[test]
pub fn test_u32() {
    assert_has_extensions(42u32);
}

#[test]
pub fn test_i32() {
    assert_has_extensions(42i32);
}

#[test]
pub fn test_u64() {
    assert_has_extensions(42u64);
}

#[test]
pub fn test_i64() {
    assert_has_extensions(42i64);
}

#[test]
pub fn test_usize() {
    assert_has_extensions(42usize);
}
