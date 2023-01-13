#![feature(specialization)]

#[macro_use]
extern crate phlow;

use phlow::{PhlowObject, PhlowView};

define_extensions!(ExampleExtensions);

#[phlow::extensions(ExampleExtensions, i32)]
impl I32Extensions {
    #[phlow::view]
    fn hex_for(_this: &i32, view: impl PhlowView) -> impl PhlowView {
        view.list()
            .title("Info")
            .items(|number: &i32, _object| {
                phlow_all!(vec![
                    ("Decimal", phlow!(number.clone())),
                    ("Hex", phlow!(format!("{:X}", number))),
                    ("Octal", phlow!(format!("{:o}", number))),
                    ("Binary", phlow!(format!("{:b}", number)))
                ])
            })
            .item_text(|each: &(&str, PhlowObject), _object| format!("{}: {}", each.0, each.1.to_string()))
            .send(|each: &(&str, PhlowObject), _object| each.1.clone())
    }
}

import_extensions!(ExampleExtensions);
fn main() {
    let number: i32 = 42;

    let view = phlow!(number).phlow_view_named("hex_for").unwrap();
    println!("{}", view);
}
