#![feature(min_specialization)]

#[macro_use]
extern crate phlow;

use phlow_extensions::CoreExtensions;

import_extensions!(CoreExtensions);

fn main() {
    phlow_server::serve(phlow!("Hello".to_string()))
        .join()
        .expect("Failed to spawn phlow server");
}
