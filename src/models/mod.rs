pub mod post;
pub mod collection;
pub mod incident;
pub mod geo_capture;
pub mod route;
pub mod sequence;

use crate::traits::Validatable;

use super::{
    MapkyAppCollection, MapkyAppGeoCapture, MapkyAppIncident, MapkyAppPost, MapkyAppRoute,
    MapkyAppSequence,
};

/// A unified enum wrapping all MapkyApp objects.
#[derive(Debug, Clone)]
pub enum MapkyAppObject {
    Post(post::MapkyAppPost),
    Collection(collection::MapkyAppCollection),
    Incident(incident::MapkyAppIncident),
    GeoCapture(geo_capture::MapkyAppGeoCapture),
    Route(route::MapkyAppRoute),
    Sequence(sequence::MapkyAppSequence),
}

impl MapkyAppObject {
    /// Parse a blob into a MapkyAppObject based on the path segment.
    /// path_segment should be e.g. "posts", "collections", etc.
    pub fn from_path(path_segment: &str, blob: &[u8], id: &str) -> Result<Self, String> {
        match path_segment {
            "posts" => {
                let obj = <MapkyAppPost as Validatable>::try_from(blob, id)?;
                Ok(MapkyAppObject::Post(obj))
            }
            "collections" => {
                let obj = <MapkyAppCollection as Validatable>::try_from(blob, id)?;
                Ok(MapkyAppObject::Collection(obj))
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
