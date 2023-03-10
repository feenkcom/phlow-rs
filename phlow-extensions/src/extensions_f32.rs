use phlow::{PhlowObject, PhlowView};

#[phlow::extensions(CoreExtensions, f32)]
impl F32Extensions {
    #[phlow::view]
    fn info_for(_this: &f32, view: impl PhlowView) -> impl PhlowView {
        view.columned_list()
            .title("Info")
            .priority(5)
            .items::<f32>(|number| {
                phlow_all!(vec![
                    ("Float", phlow!(number.clone())),
                    ("Fract", phlow!(number.fract())),
                    ("Trunk", phlow!(number.trunc())),
                    ("Bits", phlow!(format!("{:b}", number.to_bits()))),
                ])
            })
            .column(|column| {
                column
                    .title("Representation")
                    .item::<(&str, PhlowObject)>(|each| phlow!(each.0.clone()))
            })
            .column(|column| {
                column
                    .title("Value")
                    .item::<(&str, PhlowObject)>(|each| phlow!(each.1.clone()))
            })
            .send::<(&str, PhlowObject)>(|each| each.1.clone())
    }
}
