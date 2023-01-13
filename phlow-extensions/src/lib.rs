#![feature(specialization)]

#[macro_use]
extern crate phlow;

mod extensions_i32;
mod extensions_f32;
mod extensions_string;
mod extensions_vec;
mod extensions_usize;

define_extensions!(CoreExtensions);
import_extensions!(CoreExtensions);