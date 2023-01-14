#![feature(specialization)]

#[macro_use]
extern crate phlow;

use std::fmt::Debug;

use phlow::PhlowView;

#[derive(Debug)]
pub struct Generic<T: Debug> {
    value: Option<T>,
}

impl<T: Debug> Generic<T> {
    pub fn new(value: T) -> Self {
        Self { value: Some(value) }
    }
}

define_extensions!(ExampleExtensions);

#[phlow::extensions(ExampleExtensions, "Generic<T>")]
impl<T: Debug + 'static> GenericExtensions<T> {
    #[phlow::view]
    pub fn type_for(_this: &Generic<T>, view: impl PhlowView) -> impl PhlowView {
        view.list()
            .title("Type")
            .items(|generic: &Generic<T>, object| {
                phlow_all!(vec![
                    format!("Type: {}", object.value_type_name()),
                    format!("Is some: {}", generic.value.is_some()),
                    format!("Debug: {:?}", object.to_string()),
                ])
            })
    }
}

import_extensions!(ExampleExtensions);
fn main() {
    let value = Generic::<u32>::new(42);

    let view = phlow!(value).phlow_view_named("type_for").unwrap();
    println!("{}", view);
}
