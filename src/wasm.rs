use crate::traits::{HasIdPath, TimestampId, Validatable};
use crate::*;
use pubky_app_specs::traits::HashId;
use pubky_app_specs::PubkyAppTag;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct MapkyMeta {
    id: String,
    path: String,
    url: String,
}

#[wasm_bindgen]
impl MapkyMeta {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn path(&self) -> String {
        self.path.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn url(&self) -> String {
        self.url.clone()
    }
}

impl MapkyMeta {
    pub fn from_object(id: &str, pubky_id: &PubkyId, path: String) -> Self {
        Self {
            id: id.to_string(),
            url: format!("{}{}{}", PROTOCOL, pubky_id, path),
            path,
        }
    }
}

macro_rules! result_struct {
    ($struct_name:ident, $field_name:ident, $field_type:ty) => {
        #[wasm_bindgen]
        pub struct $struct_name {
            $field_name: $field_type,
            meta: MapkyMeta,
        }

        #[wasm_bindgen]
        impl $struct_name {
            #[wasm_bindgen(getter)]
            pub fn $field_name(&self) -> $field_type {
                self.$field_name.clone()
            }

            #[wasm_bindgen(getter)]
            pub fn meta(&self) -> MapkyMeta {
                self.meta.clone()
            }
        }
    };
}

result_struct!(MapkyPostResult, post, MapkyAppPost);
result_struct!(MapkyTagResult, tag, PubkyAppTag);
result_struct!(MapkyCollectionResult, collection, MapkyAppCollection);
result_struct!(MapkyIncidentResult, incident, MapkyAppIncident);
result_struct!(MapkyGeoCaptureResult, geo_capture, MapkyAppGeoCapture);
result_struct!(MapkyRouteResult, route, MapkyAppRoute);

#[wasm_bindgen]
pub struct MapkySpecsBuilder {
    #[wasm_bindgen(skip)]
    pubky_id: PubkyId,
}

#[wasm_bindgen]
impl MapkySpecsBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new(pubky_id: String) -> Result<Self, String> {
        let pubky_id = PubkyId::try_from(&pubky_id)?;
        Ok(Self { pubky_id })
    }

    #[wasm_bindgen(js_name = createPost)]
    pub fn create_post(
        &self,
        kind: MapkyAppPostKind,
        place: String,
        content: Option<String>,
        rating: Option<u8>,
        attachments: Option<Vec<String>>,
        parent: Option<String>,
    ) -> Result<MapkyPostResult, String> {
        let post = MapkyAppPost::new(kind, place, content, rating, attachments, parent);
        let post_id = post.create_id();
        post.validate(Some(&post_id))?;

        let path = MapkyAppPost::create_path(&post_id);
        let meta = MapkyMeta::from_object(&post_id, &self.pubky_id, path);

        Ok(MapkyPostResult { post, meta })
    }

    /// Create a PubkyAppTag for tagging an OSM place (or any URI).
    /// The tag is stored at `/pub/mapky.app/tags/{tag_id}` — the mapky-specific
    /// path that triggers universal tag indexing in pubky-nexus.
    #[wasm_bindgen(js_name = createPlaceTag)]
    pub fn create_place_tag(
        &self,
        uri: String,
        label: String,
    ) -> Result<MapkyTagResult, String> {
        let tag = PubkyAppTag::new(uri, label);
        let tag_id = tag.create_id();
        let path = format!("/pub/mapky.app/tags/{}", tag_id);
        let meta = MapkyMeta::from_object(&tag_id, &self.pubky_id, path);
        Ok(MapkyTagResult { tag, meta })
    }

    #[wasm_bindgen(js_name = createCollection)]
    pub fn create_collection(
        &self,
        name: String,
        description: Option<String>,
        items: JsValue,
        image_uri: Option<String>,
    ) -> Result<MapkyCollectionResult, String> {
        let items_vec: Vec<String> = from_value(items).map_err(|e| e.to_string())?;
        let collection = MapkyAppCollection::new(name, description, items_vec, image_uri);
        let collection_id = collection.create_id();
        collection.validate(Some(&collection_id))?;

        let path = MapkyAppCollection::create_path(&collection_id);
        let meta = MapkyMeta::from_object(&collection_id, &self.pubky_id, path);

        Ok(MapkyCollectionResult { collection, meta })
    }

    #[wasm_bindgen(js_name = createIncident)]
    pub fn create_incident(
        &self,
        incident_type: IncidentType,
        severity: IncidentSeverity,
        lat: f64,
        lon: f64,
    ) -> Result<MapkyIncidentResult, String> {
        let incident = MapkyAppIncident::new(incident_type, severity, lat, lon);
        let incident_id = incident.create_id();
        incident.validate(Some(&incident_id))?;

        let path = MapkyAppIncident::create_path(&incident_id);
        let meta = MapkyMeta::from_object(&incident_id, &self.pubky_id, path);

        Ok(MapkyIncidentResult { incident, meta })
    }

    #[wasm_bindgen(js_name = createGeoCapture)]
    pub fn create_geo_capture(
        &self,
        file_uri: String,
        kind: GeoCaptureKind,
        lat: f64,
        lon: f64,
    ) -> Result<MapkyGeoCaptureResult, String> {
        let capture = MapkyAppGeoCapture::new(file_uri, kind, lat, lon);
        let capture_id = capture.create_id();
        capture.validate(Some(&capture_id))?;

        let path = MapkyAppGeoCapture::create_path(&capture_id);
        let meta = MapkyMeta::from_object(&capture_id, &self.pubky_id, path);

        Ok(MapkyGeoCaptureResult {
            geo_capture: capture,
            meta,
        })
    }

    #[wasm_bindgen(js_name = createRoute)]
    pub fn create_route(
        &self,
        name: String,
        activity: RouteActivityType,
        waypoints: JsValue,
    ) -> Result<MapkyRouteResult, String> {
        let waypoints_vec: Vec<Waypoint> = from_value(waypoints).map_err(|e| e.to_string())?;
        let route = MapkyAppRoute::new(name, activity, waypoints_vec);
        let route_id = route.create_id();
        route.validate(Some(&route_id))?;

        let path = MapkyAppRoute::create_path(&route_id);
        let meta = MapkyMeta::from_object(&route_id, &self.pubky_id, path);

        Ok(MapkyRouteResult { route, meta })
    }
}
