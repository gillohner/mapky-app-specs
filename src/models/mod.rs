pub mod geo_capture;
pub mod incident;
pub mod review;
pub mod route;
pub mod sequence;

use crate::traits::Validatable;

use super::{
    MapkyAppGeoCapture, MapkyAppIncident, MapkyAppReview, MapkyAppRoute, MapkyAppSequence,
};

/// A unified enum wrapping all MapkyApp objects.
#[derive(Debug, Clone)]
pub enum MapkyAppObject {
    Review(review::MapkyAppReview),
    Incident(incident::MapkyAppIncident),
    GeoCapture(geo_capture::MapkyAppGeoCapture),
    Route(route::MapkyAppRoute),
    Sequence(sequence::MapkyAppSequence),
}

impl MapkyAppObject {
    /// Parse a blob into a MapkyAppObject based on the path segment.
    /// path_segment should be e.g. "reviews", "incidents", etc.
    /// Note: posts at "posts/" use `pubky_app_specs::PubkyAppPost` directly
    /// and are not represented in this enum.
    pub fn from_path(path_segment: &str, blob: &[u8], id: &str) -> Result<Self, String> {
        match path_segment {
            "reviews" => {
                let obj = <MapkyAppReview as Validatable>::try_from(blob, id)?;
                Ok(MapkyAppObject::Review(obj))
            }
            "incidents" => {
                let obj = <MapkyAppIncident as Validatable>::try_from(blob, id)?;
                Ok(MapkyAppObject::Incident(obj))
            }
            "geo_captures" => {
                let obj = <MapkyAppGeoCapture as Validatable>::try_from(blob, id)?;
                Ok(MapkyAppObject::GeoCapture(obj))
            }
            "routes" => {
                let obj = <MapkyAppRoute as Validatable>::try_from(blob, id)?;
                Ok(MapkyAppObject::Route(obj))
            }
            "sequences" => {
                let obj = <MapkyAppSequence as Validatable>::try_from(blob, id)?;
                Ok(MapkyAppObject::Sequence(obj))
            }
            _ => Err(format!("Unrecognized mapky.app resource: {}", path_segment)),
        }
    }
}
