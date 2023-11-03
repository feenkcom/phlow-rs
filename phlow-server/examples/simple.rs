#![feature(min_specialization)]

#[macro_use]
extern crate phlow;

use phlow_extensions::CoreExtensions;
use phlow_server::PhlowServer;

import_extensions!(CoreExtensions);

fn main() {
    let server = PhlowServer::new(phlow!("Hello".to_string()));
    server.register_object(phlow!(42));
    server.register_object(phlow!("World".to_string()));
    server.register_object(phlow!(3.14));

    phlow_server::spawn(server, 1234)
        .join()
        .expect("Failed to spawn phlow server");
}
