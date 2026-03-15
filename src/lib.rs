mod common;
pub mod constants;
mod models;
pub mod traits;
mod validation;
mod utils;

// Re-export constants
pub use constants::{
    MAPKY_PATH, MAX_ATTACHMENT_URL_LENGTH, MAX_ATTACHMENTS, MAX_COLLECTION_ITEMS,
    MAX_COLLECTION_NAME_LENGTH, MAX_CONTENT_LENGTH, MAX_DESCRIPTION_LENGTH,
    MAX_INCIDENT_ATTACHMENTS, MAX_INCIDENT_DESCRIPTION_LENGTH, MAX_ROUTE_CONTROL_POINTS,
    MAX_ROUTE_DESCRIPTION_LENGTH, MAX_ROUTE_INSTRUCTION_LENGTH, MAX_ROUTE_NAME_LENGTH,
    MAX_ROUTE_WAYPOINTS, MAX_TAG_LABEL_LENGTH, MAX_WAYPOINT_NAME_LENGTH, MIN_TAG_LABEL_LENGTH,
    MIN_WAYPOINTS, PROTOCOL, PUBLIC_PATH,
};

// Re-export domain types
pub use models::osm_ref::{OsmElementType, OsmRef};
pub use models::post::MapkyAppPost;
pub use models::location_tag::MapkyAppLocationTag;
pub use models::collection::MapkyAppCollection;
pub use models::incident::{IncidentSeverity, IncidentType, MapkyAppIncident};
pub use models::geo_capture::{GeoCaptureKind, MapkyAppGeoCapture};
pub use models::route::{MapkyAppRoute, RouteActivityType, RouteDifficulty, RouteStep, Waypoint};
pub use models::MapkyAppObject;

// Re-export from pubky-app-specs
pub use pubky_app_specs::PubkyId;

// Re-export utils
pub use utils::*;
pub use validation::*;

// WASM module
#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use wasm::*;
