use phlow::{PhlowObject, PhlowView};

#[phlow::extensions(CoreExtensions, String)]
impl StringExtensions {
    #[phlow::view]
    fn print_for(_this: &String, view: impl PhlowView) -> impl PhlowView {
        view.list()
            .title("Print")
            .priority(5)
            .items(|string: &String, object| {
                phlow_all!(vec![string.clone()])
            })
            .item_text(|each: &String, _object | each.to_string())
            .send(|each: &String, object| phlow!(each.clone()))
    }

    #[phlow::view]
    fn chars_for(_this: &String, view: impl PhlowView) -> impl PhlowView {
        view.list()
            .title("Chars")
            .priority(6)
            .items(|string: &String, object| {
                phlow_all!(string.chars())
            })
            .item_text(|each: &char, _object | each.to_string())
            .send(|each: &char, object| phlow!(each.clone()))
    }
}
