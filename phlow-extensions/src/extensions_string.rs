use phlow::PhlowView;

#[phlow::extensions(CoreExtensions, String)]
impl StringExtensions {
    #[phlow::view]
    fn print_for(_this: &String, view: impl PhlowView) -> impl PhlowView {
        view.list()
            .title("Print")
            .priority(5)
            .items(|string: &String, _object| phlow_all!(vec![string.clone()]))
    }

    #[phlow::view]
    fn chars_for(_this: &String, view: impl PhlowView) -> impl PhlowView {
        view.list()
            .title("Chars")
            .priority(6)
            .items(|string: &String, _object| phlow_all!(string.chars()))
    }
}
