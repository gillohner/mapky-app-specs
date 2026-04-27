use crate::{
    constants::{MAPKY_PATH, PROTOCOL, PUBLIC_PATH},
    traits::HasIdPath,
    MapkyAppCollection, MapkyAppGeoCapture, MapkyAppIncident, MapkyAppPost, MapkyAppRoute,
    MapkyAppSequence,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Builds a Mapky base URI: "pubky://<user_id>/pub/mapky.app/"
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = mapkyBaseUriBuilder))]
pub fn mapky_base_uri_builder(user_id: String) -> String {
    format!("{}{}{}{}", PROTOCOL, user_id, PUBLIC_PATH, MAPKY_PATH)
}

/// Builds a Post URI: "pubky://<author_id>/pub/mapky.app/posts/<post_id>"
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = mapkyPostUriBuilder))]
pub fn mapky_post_uri_builder(author_id: String, post_id: String) -> String {
    let path = MapkyAppPost::create_path(&post_id);
    [PROTOCOL, &author_id, &path].concat()
}

/// Builds a Collection URI
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = mapkyCollectionUriBuilder))]
pub fn mapky_collection_uri_builder(author_id: String, collection_id: String) -> String {
    let path = MapkyAppCollection::create_path(&collection_id);
    [PROTOCOL, &author_id, &path].concat()
}

/// Builds an Incident URI
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = mapkyIncidentUriBuilder))]
pub fn mapky_incident_uri_builder(author_id: String, incident_id: String) -> String {
    let path = MapkyAppIncident::create_path(&incident_id);
    [PROTOCOL, &author_id, &path].concat()
}

/// Builds a GeoCapture URI
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = mapkyGeoCaptureUriBuilder))]
pub fn mapky_geo_capture_uri_builder(author_id: String, capture_id: String) -> String {
    let path = MapkyAppGeoCapture::create_path(&capture_id);
    [PROTOCOL, &author_id, &path].concat()
}

/// Builds a Route URI
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = mapkyRouteUriBuilder))]
pub fn mapky_route_uri_builder(author_id: String, route_id: String) -> String {
    let path = MapkyAppRoute::create_path(&route_id);
    [PROTOCOL, &author_id, &path].concat()
}

/// Builds a Sequence URI
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = mapkySequenceUriBuilder))]
pub fn mapky_sequence_uri_builder(author_id: String, sequence_id: String) -> String {
    let path = MapkyAppSequence::create_path(&sequence_id);
    [PROTOCOL, &author_id, &path].concat()
}
