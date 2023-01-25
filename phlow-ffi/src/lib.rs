#![allow(non_snake_case)]
#![allow(incomplete_features)]
#![feature(specialization)]

extern crate phlow;

pub use phlow_columned_list_view::*;
pub use phlow_list_view::*;
pub use phlow_object::*;
pub use phlow_text_view::*;
pub use phlow_view::*;
pub use phlow_view_method::*;

mod phlow_columned_list_view;
mod phlow_list_view;
mod phlow_object;
mod phlow_text_view;
mod phlow_view;
mod phlow_view_method;

#[no_mangle]
pub fn phlow_init_env_logger() {
    env_logger::init();
}
