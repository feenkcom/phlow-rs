use phlow::{phlow, phlow_all, PhlowView};

use crate::PhlowServer;

#[phlow::extensions(PhlowServerExtensions, PhlowServer)]
impl ServerExtensions {
    #[phlow::view]
    fn info_for(_this: &PhlowServer, view: impl PhlowView) -> impl PhlowView {
        view.columned_list()
            .title("Info")
            .priority(5)
            .items::<PhlowServer>(|server| {
                phlow_all!(vec![
                    ("Session", phlow!(server.session())),
                    ("ObjectId", phlow!(server.id())),
                ])
            })
            .column_item::<(&str, phlow::PhlowObject)>("Property", |each| {
                phlow!(each.0.to_string())
            })
            .column_item::<(&str, phlow::PhlowObject)>("Value", |each| phlow!(each.1.clone()))
            .send::<(&str, phlow::PhlowObject)>(|each| each.1.clone())
    }

    #[phlow::view]
    fn routes_for(_this: &PhlowServer, view: impl PhlowView) -> impl PhlowView {
        view.columned_list()
            .title("Routes")
            .priority(6)
            .items::<PhlowServer>(|server| phlow_all!(server.get_routes()))
            .column_item::<(String, String)>("Mode", |each| phlow!(each.0.clone()))
            .column_item::<(String, String)>("Path", |each| phlow!(each.1.clone()))
    }
}
