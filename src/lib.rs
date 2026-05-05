mod common;
pub mod constants;
mod models;
pub mod traits;
mod utils;
mod validation;

pub use common::BoundingBox;

// Re-export constants
pub use constants::{
    MAPKY_PATH, MAX_ATTACHMENTS, MAX_ATTACHMENT_URL_LENGTH, MAX_COLLECTION_ITEMS,
    MAX_COLLECTION_NAME_LENGTH, MAX_CONTENT_LENGTH, MAX_DESCRIPTION_LENGTH,
    MAX_INCIDENT_ATTACHMENTS, MAX_INCIDENT_DESCRIPTION_LENGTH, MAX_ROUTE_CONTROL_POINTS,
    MAX_ROUTE_DESCRIPTION_LENGTH, MAX_ROUTE_INSTRUCTION_LENGTH, MAX_ROUTE_NAME_LENGTH,
    MAX_ROUTE_WAYPOINTS, MAX_SEQUENCE_CAPTURE_COUNT, MAX_SEQUENCE_DESCRIPTION_LENGTH,
    MAX_SEQUENCE_DEVICE_LENGTH, MAX_SEQUENCE_NAME_LENGTH, MAX_WAYPOINT_NAME_LENGTH, MIN_WAYPOINTS,
    PROTOCOL, PUBLIC_PATH,
};

// Re-export domain types
pub use models::collection::MapkyAppCollection;
pub use models::geo_capture::{GeoCaptureKind, MapkyAppGeoCapture};
pub use models::incident::{IncidentSeverity, IncidentType, MapkyAppIncident};
pub use models::review::MapkyAppReview;
pub use models::route::{MapkyAppRoute, RouteActivityType, RouteGeometry, RouteStep, Waypoint};
pub use models::sequence::MapkyAppSequence;
pub use models::MapkyAppObject;

// Re-export from pubky-app-specs
// PubkyAppPost / PubkyAppPostKind / PubkyAppPostEmbed are reused as-is for
// generic comments stored at /pub/mapky.app/posts/{id} (cross-namespace pattern,
// like universal tags).
pub use pubky_app_specs::{PubkyAppPost, PubkyAppPostEmbed, PubkyAppPostKind, PubkyId};

// Re-export utils
pub use utils::*;
pub use validation::*;

// WASM module
#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use wasm::*;
