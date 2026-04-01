// Path constants
pub static PUBLIC_PATH: &str = "/pub/";
pub static MAPKY_PATH: &str = "mapky.app/";
pub static PROTOCOL: &str = "pubky://";

// Post limits
pub const MAX_CONTENT_LENGTH: usize = 5000;
pub const MAX_ATTACHMENTS: usize = 20;
pub const MAX_ATTACHMENT_URL_LENGTH: usize = 300;

// Collection limits
pub const MAX_COLLECTION_NAME_LENGTH: usize = 100;
pub const MAX_COLLECTION_ITEMS: usize = 500;

// Description limits (shared)
pub const MAX_DESCRIPTION_LENGTH: usize = 2000;

// Incident limits
pub const MAX_INCIDENT_DESCRIPTION_LENGTH: usize = 500;
pub const MAX_INCIDENT_ATTACHMENTS: usize = 5;

// GeoCapture limits
pub const MAX_CAPTION_LENGTH: usize = 500;

// Route limits
pub const MAX_ROUTE_NAME_LENGTH: usize = 200;
pub const MAX_ROUTE_DESCRIPTION_LENGTH: usize = 10000;
pub const MAX_ROUTE_WAYPOINTS: usize = 10000;
pub const MIN_WAYPOINTS: usize = 2;
pub const MAX_ROUTE_CONTROL_POINTS: usize = 500;
pub const MAX_ROUTE_INSTRUCTION_LENGTH: usize = 500;
pub const MAX_WAYPOINT_NAME_LENGTH: usize = 100;
