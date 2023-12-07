#![allow(incomplete_features)]
#![feature(min_specialization)]

#[macro_use]
extern crate phlow;

use phlow::{PhlowBitmap, PhlowView};

fn attenuate(myfloat: f32) -> u8 {
    if myfloat > f32::from(u8::MAX) {
        u8::MAX
    } else if myfloat < f32::from(u8::MIN) {
        u8::MIN
    } else {
        myfloat.round() as u8
    }
}

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

    pub fn as_rgba(&self) -> Vec<u8> {
        let red: u8 = attenuate(self.red);
        let green: u8 = attenuate(self.green);
        let blue: u8 = attenuate(self.blue);
        let alpha: u8 = attenuate(self.alpha);

        vec![red, green, blue, alpha]
    }
}

define_extensions!(ExampleExtensions);

#[phlow::extensions(ExampleExtensions, Color)]
impl ColorExtensions {
    #[phlow::view]
    pub fn rgba_for(_this: &Color, view: impl PhlowView) -> impl PhlowView {
        view.list()
            .title("RGBA")
            .items::<Color>(
                |color| phlow_all!(vec![color.red, color.green, color.blue, color.alpha])
            )
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

    #[phlow::view]
    pub fn preview_for(_this: &Color, view: impl PhlowView) -> impl PhlowView {
        view.bitmap().title("Preview").bitmap::<Color>(|color| {
            let width = 64i32;
            let height = 64i32;
            let pixels: Vec<u8> = color
                .as_rgba()
                .into_iter()
                .cycle()
                .take((width * height * 4) as usize)
                .collect();
            PhlowBitmap::rgba8(pixels, width, height)
        })
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

    let view = phlow!(color.clone())
        .phlow_view_named("preview_for")
        .unwrap();
    println!("{}", view);
}
