use phlow::PhlowView;

#[phlow::extensions(CoreExtensions, String)]
impl StringExtensions {
    #[phlow::view]
    fn print_for(_this: &String, view: impl PhlowView) -> impl PhlowView {
        view.text()
            .title("Print")
            .priority(5)
            .text::<String>(|string| string.to_owned())
    }

    #[phlow::view]
    fn chars_for(_this: &String, view: impl PhlowView) -> impl PhlowView {
        view.list()
            .title("Chars")
            .priority(6)
            .items::<String>(|string| phlow_all!(string.chars()))
    }
}
