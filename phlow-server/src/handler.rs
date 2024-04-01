use std::str::FromStr;

use uuid::Uuid;
use warp::{reply, Rejection, Reply};

use phlow::{PhlowObject, PhlowObjectId, PhlowViewSpecification};

use crate::{PhlowObjectDescription, PhlowServer, PhlowViewSpecificationDataNode};

pub async fn session(server: PhlowServer) -> Result<impl Reply, Rejection> {
    Ok(reply::json(&server.session().to_string()))
}

pub async fn server_id(server: PhlowServer) -> Result<impl Reply, Rejection> {
    Ok(reply::json(&server.id()))
}

pub async fn objects(server: PhlowServer) -> Result<impl Reply, Rejection> {
    Ok(reply::json(&server.inspect_objects()))
}

pub async fn object(id: PhlowObjectId, server: PhlowServer) -> Result<impl Reply, Rejection> {
    Ok(reply::json(&server.retrieve_object(id)))
}

pub async fn release_object(
    session: String,
    id: PhlowObjectId,
    server: PhlowServer,
) -> Result<impl Reply, Rejection> {
    let session_uuid = Uuid::from_str(session.as_str()).unwrap();
    let probably_deleted_object = server.release_object(session_uuid, id);
    Ok(reply::json(&probably_deleted_object.is_some()))
}

pub async fn object_views(id: PhlowObjectId, server: PhlowServer) -> Result<impl Reply, Rejection> {
    let views = server
        .registered_object_description_by_id_views(id)
        .unwrap_or_else(|| vec![]);
    let specs = views
        .into_iter()
        .map(|view| view.as_view_specification())
        .filter(|view| view.is_some())
        .map(|view| view.unwrap())
        .collect::<Vec<Box<dyn PhlowViewSpecification>>>();

    Ok(reply::json(&specs))
}

pub async fn object_view(
    id: PhlowObjectId,
    view_selector: String,
    server: PhlowServer,
) -> Result<impl Reply, Rejection> {
    let spec = find_view_specification_for_object_id(id, view_selector.as_str(), &server);

    Ok(reply::json(&spec))
}

pub async fn object_view_items(
    id: PhlowObjectId,
    view_selector: String,
    server: PhlowServer,
) -> Result<impl Reply, Rejection> {
    let spec = find_view_specification_for_object_id(id, view_selector.as_str(), &server);
    if let Some(spec) = spec {
        let items: Vec<PhlowViewSpecificationDataNode> = spec
            .retrieve_items()
            .await
            .into_iter()
            .map(|item| {
                let object = item.phlow_object().clone();
                PhlowViewSpecificationDataNode {
                    phlow_object: server.register_object(object.clone()),
                    node_id: object.object_id(),
                    node_value: item,
                }
            })
            .collect();

        return Ok(reply::json(&items));
    }
    return Ok(reply::json(&None::<Vec<PhlowViewSpecificationDataNode>>));
}

pub async fn object_view_sent_item(
    inspected_object_id: PhlowObjectId,
    view_selector: String,
    selected_object_id: PhlowObjectId,
    server: PhlowServer,
) -> Result<impl Reply, Rejection> {
    let none_reply = Ok(reply::json(&None::<PhlowObjectDescription>));

    let inspected_object = match server.find_object(inspected_object_id) {
        None => {
            return none_reply;
        }
        Some(object) => object,
    };

    let view_spec =
        match find_view_specification_for_object(&inspected_object, view_selector.as_str()) {
            None => {
                return none_reply;
            }
            Some(spec) => spec,
        };

    let selected_object = match server.find_object(selected_object_id) {
        None => return none_reply,
        Some(object) => object,
    };

    let object_to_send = match view_spec.retrieve_sent_item(&selected_object).await {
        None => return none_reply,
        Some(object) => object,
    };

    let object_description = server.register_object(object_to_send);
    Ok(reply::json(&object_description))
}

fn find_view_specification_for_object_id(
    id: PhlowObjectId,
    view_selector: &str,
    server: &PhlowServer,
) -> Option<Box<dyn PhlowViewSpecification>> {
    server
        .find_object(id)
        .and_then(|object| find_view_specification_for_object(&object, view_selector))
}

fn find_view_specification_for_object(
    object: &PhlowObject,
    view_selector: &str,
) -> Option<Box<dyn PhlowViewSpecification>> {
    let views = object.phlow_views();
    let view = views
        .into_iter()
        .find(|each| each.get_defining_method().full_method_name.as_str() == view_selector);
    view.and_then(|view| view.as_view_specification())
}
