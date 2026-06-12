use crate::traits::{HasIdPath, TimestampId, Validatable};
use crate::*;
use pubky_app_specs::traits::HashId;
use pubky_app_specs::PubkyAppTag;
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

result_struct!(MapkyReviewResult, review, MapkyAppReview);
result_struct!(MapkyPostResult, post, PubkyAppPost);
result_struct!(MapkyTagResult, tag, PubkyAppTag);
result_struct!(MapkyIncidentResult, incident, MapkyAppIncident);
result_struct!(MapkyGeoCaptureResult, geo_capture, MapkyAppGeoCapture);
result_struct!(MapkyRouteResult, route, MapkyAppRoute);
result_struct!(MapkySequenceResult, sequence, MapkyAppSequence);

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

    /// Create a `MapkyAppReview` (rating-mandatory, place-anchored, never a reply).
    /// Stored at `/pub/mapky.app/reviews/{id}`.
    #[wasm_bindgen(js_name = createReview)]
    pub fn create_review(
        &self,
        place: String,
        rating: u8,
        content: Option<String>,
        attachments: Option<Vec<String>>,
    ) -> Result<MapkyReviewResult, String> {
        let review = MapkyAppReview::new(place, rating, content, attachments);
        let review_id = review.create_id();
        review.validate(Some(&review_id))?;

        let path = MapkyAppReview::create_path(&review_id);
        let meta = MapkyMeta::from_object(&review_id, &self.pubky_id, path);

        Ok(MapkyReviewResult { review, meta })
    }

    /// Create a `PubkyAppPost` (generic comment / threaded reply) stored under
    /// the MapKy namespace at `/pub/mapky.app/posts/{id}`. The `parent` field
    /// can target any MapKy resource (review, route, geo-capture, sequence,
    /// incident, or another mapky-namespaced post). Cross-domain parents
    /// (e.g. core social posts) are accepted but only edge-indexed when the
    /// target is a MapKy resource.
    #[wasm_bindgen(js_name = createMapkyPost)]
    pub fn create_mapky_post(
        &self,
        content: String,
        kind: PubkyAppPostKind,
        parent: Option<String>,
        embed: Option<PubkyAppPostEmbed>,
        attachments: Option<Vec<String>>,
    ) -> Result<MapkyPostResult, String> {
        let post = PubkyAppPost::new(content, kind, parent, embed, attachments);
        let post_id = post.create_id();
        post.validate(Some(&post_id))?;

        // Override the default `/pub/pubky.app/posts/{id}` path with the
        // mapky-namespaced equivalent so the plugin's namespace claim picks it up.
        let path = format!("{}{}posts/{}", PUBLIC_PATH, MAPKY_PATH, post_id);
        let meta = MapkyMeta::from_object(&post_id, &self.pubky_id, path);

        Ok(MapkyPostResult { post, meta })
    }

    /// Create a PubkyAppTag for tagging an OSM place (or any URI).
    /// The tag is stored at `/pub/mapky.app/tags/{tag_id}` — the mapky-specific
    /// path that triggers universal tag indexing in pubky-nexus.
    #[wasm_bindgen(js_name = createPlaceTag)]
    pub fn create_place_tag(&self, uri: String, label: String) -> Result<MapkyTagResult, String> {
        let tag = PubkyAppTag::new(uri, label);
        let tag_id = tag.create_id();
        let path = format!("/pub/mapky.app/tags/{}", tag_id);
        let meta = MapkyMeta::from_object(&tag_id, &self.pubky_id, path);
        Ok(MapkyTagResult { tag, meta })
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

    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(js_name = createGeoCapture)]
    pub fn create_geo_capture(
        &self,
        file_uri: String,
        kind: GeoCaptureKind,
        lat: f64,
        lon: f64,
        ele: Option<f64>,
        heading: Option<f64>,
        pitch: Option<f64>,
        fov: Option<f64>,
        caption: Option<String>,
        captured_at: Option<i64>,
    ) -> Result<MapkyGeoCaptureResult, String> {
        let mut capture = MapkyAppGeoCapture::new(file_uri, kind, lat, lon);
        capture.ele = ele;
        capture.heading = heading;
        capture.pitch = pitch;
        capture.fov = fov;
        capture.caption = caption;
        capture.captured_at = captured_at;

        let capture = capture.sanitize();
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

    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(js_name = createSequence)]
    pub fn create_sequence(
        &self,
        kind: GeoCaptureKind,
        captured_at_start: i64,
        captured_at_end: i64,
        capture_count: u32,
        name: Option<String>,
        description: Option<String>,
        device: Option<String>,
        min_lat: Option<f64>,
        min_lon: Option<f64>,
        max_lat: Option<f64>,
        max_lon: Option<f64>,
    ) -> Result<MapkySequenceResult, String> {
        let mut sequence =
            MapkyAppSequence::new(kind, captured_at_start, captured_at_end, capture_count);
        sequence.name = name;
        sequence.description = description;
        sequence.device = device;

        if let (Some(min_lat), Some(min_lon), Some(max_lat), Some(max_lon)) =
            (min_lat, min_lon, max_lat, max_lon)
        {
            sequence.bbox = Some(BoundingBox {
                min_lat,
                min_lon,
                max_lat,
                max_lon,
            });
        }

        let sequence = sequence.sanitize();
        let sequence_id = sequence.create_id();
        sequence.validate(Some(&sequence_id))?;

        let path = MapkyAppSequence::create_path(&sequence_id);
        let meta = MapkyMeta::from_object(&sequence_id, &self.pubky_id, path);

        Ok(MapkySequenceResult { sequence, meta })
    }
}
