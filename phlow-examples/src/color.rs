#![allow(incomplete_features)]
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
            .items::<Color>(|color| {
                phlow_all!(vec![color.red, color.green, color.blue, color.alpha])
            })
            .item_text::<f32>(|each| each.to_string())
            .send::<f32>(|each| phlow!(each.clone()))
    }

    #[phlow::view]
    pub fn components_for(_this: &Color, view: impl PhlowView) -> impl PhlowView {
        view.columned_list()
            .title("RGBA Components")
            .items::<Color>(|color| {
                phlow_all!(vec![
                    ("Red", color.red),
                    ("Green", color.green),
                    ("Blue", color.blue),
                    ("Alpha", color.alpha)
                ])
            })
            .column(|column| {
                column
                    .title("Component")
                    .item::<(&str, f32)>(|each| phlow!(each.0))
            })
            .column_item::<(&str, f32)>("Value", |each| phlow!(each.1))
            .send::<(&str, f32)>(|each| phlow!(each.1))
    }
}

import_extensions!(ExampleExtensions);
fn main() {
    let color = Color::new();

    let view = phlow!(color.clone()).phlow_view_named("rgba_for").unwrap();
    println!("{}", view);

    let view = phlow!(color.clone())
        .phlow_view_named("components_for")
        .unwrap();
    println!("{}", view);
}
