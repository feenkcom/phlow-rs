#![feature(min_specialization)]

#[macro_use]
extern crate phlow;

use phlow::PhlowView;

#[derive(Clone)]
pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
}

impl Color {
    pub fn new() -> Self {
        Self {
            red: 1.0,
            green: 0.5,
            blue: 0.75,
            alpha: 1.0,
        }
    }
}

define_extensions!(ExampleExtensions);

#[phlow::extensions(ExampleExtensions, Color)]
impl ColorExtensions {
    #[phlow::view]
    pub fn rgba_for(_this: &Color, view: impl PhlowView) -> impl PhlowView {
        view.list()
            .title("RGBA")
            .items(|color: &Color, object| {
                phlow_all!(vec![color.red, color.green, color.blue, color.alpha])
            })
            .item_text(|each: &f32, object| each.to_string())
    }
}

import_extensions!(ExampleExtensions);
fn main() {
    let color = Color::new();

    let view = phlow!(color).phlow_view_named("rgba_for").unwrap();
    println!("{}", view)
}
