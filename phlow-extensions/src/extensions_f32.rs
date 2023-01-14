use phlow::{PhlowObject, PhlowView};

#[phlow::extensions(CoreExtensions, f32)]
impl F32Extensions {
    #[phlow::view]
    fn info_for(_this: &f32, view: impl PhlowView) -> impl PhlowView {
        view.list()
            .title("Info")
            .priority(5)
            .items(|number: &f32, _object| {
                phlow_all!(vec![
                    ("Float", phlow!(number.clone())),
                    ("Rounded", phlow!(number.round() as i32)),
                ])
            })
            .item_text(|each: &(&str, PhlowObject), _object| {
                format!("{}: {}", each.0, each.1.to_string())
            })
            .send(|each: &(&str, PhlowObject), _object| each.1.clone())
    }
}
