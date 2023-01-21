use phlow::{PhlowObject, PhlowView};
use std::rc::Rc;

#[phlow::extensions(CoreExtensions, "Rc<T>")]
impl<T: 'static> RcExtensions<T> {
    #[phlow::view]
    fn info_for(_this: &Rc<T>, view: impl PhlowView) -> impl PhlowView {
        view.columned_list()
            .title("Info")
            .priority(5)
            .items::<Rc<T>>(|reference| {
                phlow_all!(vec![
                    ("Strong count", phlow!(Rc::strong_count(&reference))),
                    ("Weak count", phlow!(Rc::weak_count(&reference))),
                ])
            })
            .column(|column| {
                column
                    .title("Key")
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
