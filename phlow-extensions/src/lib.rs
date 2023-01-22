#![allow(incomplete_features)]
#![feature(specialization)]

#[macro_use]
extern crate phlow;

mod extensions_f32;
mod extensions_integer;
mod extensions_rc;
mod extensions_string;
mod extensions_vec;

define_extensions!(CoreExtensions);
import_extensions!(CoreExtensions);

#[macro_export]
macro_rules! representations_view_for_integer {
    ($extension_name:ident, $integer_type:ident) => {
        #[phlow::extensions(CoreExtensions, $integer_type)]
        impl $extension_name {
            #[phlow::view]
            fn representations_for(
                _this: &$integer_type,
                view: impl phlow::PhlowView,
            ) -> impl phlow::PhlowView {
                view.columned_list()
                    .title("Info")
                    .priority(5)
                    .items::<$integer_type>(|number| {
                        phlow_all!(vec![
                            ("Decimal", phlow!(number.clone())),
                            ("Hex", phlow!(format!("{:X}", number))),
                            ("Octal", phlow!(format!("{:o}", number))),
                            ("Binary", phlow!(format!("{:b}", number)))
                        ])
                    })
                    .column_item::<(&str, phlow::PhlowObject)>("Representation", |each| {
                        phlow!(each.0.clone())
                    })
                    .column_item::<(&str, phlow::PhlowObject)>("Value", |each| {
                        phlow!(each.1.clone())
                    })
                    .send::<(&str, phlow::PhlowObject)>(|each| each.1.clone())
            }
        }
    };
}
