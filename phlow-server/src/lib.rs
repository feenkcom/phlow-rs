#![feature(min_specialization)]

use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use std::thread;

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;
use uuid::Uuid;
use warp::Filter;

use phlow::{
    define_extensions, import_extensions, phlow, PhlowObject, PhlowObjectId, PhlowView,
    PhlowViewSpecificationListingItem,
};
use phlow_extensions::CoreExtensions;

mod extensions;
mod handler;

define_extensions!(PhlowServerExtensions);
import_extensions!(CoreExtensions, PhlowServerExtensions);

#[derive(Clone, Debug)]
pub struct PhlowServer(Arc<RwLock<PhlowServerData>>);

#[derive(Debug)]
struct PhlowServerData {
    root_object: PhlowObject,
    objects: HashMap<PhlowObjectId, (PhlowObject, usize)>,
    session: Uuid,
    routes: Vec<(String, String)>,
    server_object_id: PhlowObjectId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhlowObjectDescription {
    id: PhlowObjectId,
    object_type: String,
    print_string: String,
    reference_count: usize,
    should_auto_release: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PhlowViewSpecificationDataNode {
    pub phlow_object: PhlowObjectDescription,
    pub node_id: PhlowObjectId,
    pub node_value: Box<dyn PhlowViewSpecificationListingItem>,
}

impl PhlowObjectDescription {
    pub fn new(object: &PhlowObject, reference_count: usize) -> Self {
        Self {
            id: object.object_id(),
            object_type: object.phlow_type().type_name().to_string(),
            print_string: object.to_string(),
            reference_count,
            should_auto_release: true,
        }
    }

    pub fn without_auto_release(mut self) -> Self {
        self.should_auto_release = false;
        self
    }

    pub fn with_auto_release(mut self) -> Self {
        self.should_auto_release = true;
        self
    }
}

impl PhlowServer {
    pub fn new(root_object: PhlowObject) -> Self {
        let server = Self(Arc::new(RwLock::new(PhlowServerData {
            root_object: root_object.clone(),
            objects: Default::default(),
            session: Uuid::new_v4(),
            routes: vec![],
            server_object_id: 0,
        })));

        let server_phlow_object = phlow!(server.clone());
        server.0.write().server_object_id = server_phlow_object.object_id();

        server.register_object(root_object);
        server.register_object(server_phlow_object);
        server
    }

    pub fn id(&self) -> PhlowObjectId {
        self.0.read().server_object_id
    }

    pub fn session(&self) -> Uuid {
        self.0.read().session.clone()
    }

    pub fn add_route(&self, method: &str, new_route: &str) {
        self.0
            .write()
            .routes
            .push((method.to_string(), new_route.to_string()));
    }

    pub fn get_routes(&self) -> Vec<(String, String)> {
        self.0.read().routes.clone()
    }

    pub fn register_object(&self, object: PhlowObject) -> PhlowObjectDescription {
        let objects = &mut self.0.write().objects;
        let mut count = objects
            .get(&object.object_id())
            .map_or_else(|| 0, |entry| entry.1);

        count = count + 1;
        objects.insert(object.object_id(), (object.clone(), count));
        PhlowObjectDescription::new(&object, count).with_auto_release()
    }

    pub fn release_object(&self, session: Uuid, object_id: PhlowObjectId) -> Option<PhlowObject> {
        let mut lock = self.0.write();

        if session != lock.session {
            return None;
        }

        let objects = &mut lock.objects;
        if let Some(entry) = objects.get_mut(&object_id) {
            let count = entry.1 - 1;
            if count > 0 {
                entry.1 = count;
                None
            } else {
                objects.remove(&object_id).map(|entry| entry.0)
            }
        } else {
            None
        }
    }

    pub fn root_phlow_views(&self) -> Vec<Box<dyn PhlowView>> {
        self.0.read().root_object.phlow_views()
    }

    /// Return descriptions of registered objects.
    /// Doesn't increase the reference count
    pub fn inspect_objects(&self) -> Vec<PhlowObjectDescription> {
        let mut descriptions: Vec<PhlowObjectDescription> = self
            .0
            .read()
            .objects
            .values()
            .map(|object| PhlowObjectDescription::new(&object.0, object.1).without_auto_release())
            .collect();

        descriptions.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());

        descriptions
    }

    pub fn find_object(&self, id: PhlowObjectId) -> Option<PhlowObject> {
        self.0
            .read()
            .objects
            .get(&id)
            .map(|object| object.0.clone())
    }

    /// Return object description for a given object id.
    /// Increases the reference count
    pub fn retrieve_object(&self, id: PhlowObjectId) -> Option<PhlowObjectDescription> {
        let description = self.0.write().objects.get_mut(&id).map(|entry| {
            let count = entry.1 + 1;
            entry.1 = count;
            PhlowObjectDescription::new(&entry.0, entry.1).with_auto_release()
        });

        description
    }

    pub fn registered_object_description_by_id_views(
        &self,
        id: PhlowObjectId,
    ) -> Option<Vec<Box<dyn PhlowView>>> {
        self.find_object(id).map(|object| object.phlow_views())
    }
}

fn with_phlow_server(
    server: PhlowServer,
) -> impl Filter<Extract = (PhlowServer,), Error = Infallible> + Clone {
    warp::any().map(move || server.clone())
}

macro_rules! get_path {
    ($server:ident, $($pieces:tt)*) => ({
        $server.add_route("GET", stringify!($($pieces)*));
        warp::path!($($pieces)*).and(warp::get())
    });
}

macro_rules! delete_path {
    ($server:ident, $($pieces:tt)*) => ({
        $server.add_route("DELETE", stringify!($($pieces)*));
        warp::path!($($pieces)*).and(warp::delete())
    });
}

pub fn spawn(server: PhlowServer, port: u16) -> thread::JoinHandle<()> {
    let session = get_path!(server, "session")
        .and(with_phlow_server(server.clone()))
        .and_then(handler::session);

    let server_id = get_path!(server, "id")
        .and(with_phlow_server(server.clone()))
        .and_then(handler::server_id);

    let objects = get_path!(server, "objects")
        .and(with_phlow_server(server.clone()))
        .and_then(handler::objects);

    let object = get_path!(server, "objects" / PhlowObjectId)
        .and(warp::get())
        .and(with_phlow_server(server.clone()))
        .and_then(handler::object);

    let object_views = get_path!(server, "objects" / PhlowObjectId / "views")
        .and(with_phlow_server(server.clone()))
        .and_then(handler::object_views);

    let object_view = get_path!(server, "objects" / PhlowObjectId / "views" / String)
        .and(with_phlow_server(server.clone()))
        .and_then(handler::object_view);

    let object_view_items = get_path!(
        server,
        "objects" / PhlowObjectId / "views" / String / "items"
    )
    .and(with_phlow_server(server.clone()))
    .and_then(handler::object_view_items);

    let object_view_sent_item = get_path!(
        server,
        "objects" / PhlowObjectId / "views" / String / "send" / PhlowObjectId
    )
    .and(with_phlow_server(server.clone()))
    .and_then(handler::object_view_sent_item);

    let release_object = delete_path!(server, "session" / String / "objects" / PhlowObjectId)
        .and(with_phlow_server(server.clone()))
        .and_then(handler::release_object);

    let routes = session
        .or(server_id)
        .or(objects)
        .or(object)
        .or(release_object)
        .or(object_views)
        .or(object_view)
        .or(object_view_items)
        .or(object_view_sent_item);

    thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async move {
            warp::serve(routes).run(([127, 0, 0, 1], port)).await;
            ()
        });
        ()
    })
}
