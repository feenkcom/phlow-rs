use phlow::{PhlowObject, PhlowView};

#[phlow::extensions(CoreExtensions, i32)]
impl I32Extensions {
    #[phlow::view]
    fn info_for(_this: &i32, view: impl PhlowView) -> impl PhlowView {
        view.list()
            .title("Info")
            .priority(5)
            .items(|number: &i32, _object| {
                phlow_all!(vec![
                    ("Decimal", phlow!(number.clone())),
                    ("Hex", phlow!(format!("{:X}", number))),
                    ("Octal", phlow!(format!("{:o}", number))),
                    ("Binary", phlow!(format!("{:b}", number)))
                ])
            })
            .item_text(|each: &(&str, PhlowObject), _object| {
                format!("{}: {}", each.0, each.1.to_string())
            })
            .send(|each: &(&str, PhlowObject), _object| each.1.clone())
    }
}
