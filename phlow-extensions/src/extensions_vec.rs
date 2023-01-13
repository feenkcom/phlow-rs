use phlow::{PhlowObject, PhlowView};

#[phlow::extensions(CoreExtensions, "Vec<T>")]
impl<T: 'static> VecExtensions<T> {
    #[phlow::view]
    fn items_for(_this: &Vec<T>, view: impl PhlowView) -> impl PhlowView {
        view.list()
            .title("Items")
            .priority(5)
            .items(|vec: &Vec<T>, object|
                vec.iter().map(|each| phlow!(each, object)).collect())
            .send(|each: &T, object| object.clone())
    }
}
