use crate::{
    common::BoundingBox,
    constants::{
        MAX_SEQUENCE_CAPTURE_COUNT, MAX_SEQUENCE_DESCRIPTION_LENGTH, MAX_SEQUENCE_DEVICE_LENGTH,
        MAX_SEQUENCE_NAME_LENGTH,
    },
    models::geo_capture::GeoCaptureKind,
    traits::{HasIdPath, TimestampId, Validatable},
    validation::validate_timestamp_microseconds,
    MAPKY_PATH, PUBLIC_PATH,
};
use serde::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
use crate::traits::Json;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// A capture session — a coherent group of `MapkyAppGeoCapture`s uploaded together
/// (e.g. "walk down Lambeth Rd, June 2026"). GeoCaptures reference a sequence via
/// their `sequence_uri` + `sequence_index` pair.
///
/// URI: /pub/mapky.app/sequences/:sequence_id
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct MapkyAppSequence {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub name: Option<String>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub description: Option<String>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub kind: GeoCaptureKind,
    /// UNIX microseconds — earliest capture in the sequence.
    pub captured_at_start: i64,
    /// UNIX microseconds — latest capture in the sequence. Must be >= `captured_at_start`.
    pub captured_at_end: i64,
    /// Denormalized count of member captures (clients supply this; indexer stores as-is).
    pub capture_count: u32,
    /// Optional bounding box covering all member captures.
    pub bbox: Option<BoundingBox>,
    /// Free-text device identifier (e.g. "iPhone 15 Pro", "Insta360 X4").
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub device: Option<String>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl MapkyAppSequence {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(
        kind: GeoCaptureKind,
        captured_at_start: i64,
        captured_at_end: i64,
        capture_count: u32,
    ) -> Self {
        let sequence = MapkyAppSequence {
            name: None,
            description: None,
            kind,
            captured_at_start,
            captured_at_end,
            capture_count,
            bbox: None,
            device: None,
        };
        sequence.sanitize()
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl MapkyAppSequence {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromJson))]
    pub fn from_json(js_value: &JsValue) -> Result<Self, String> {
        Self::import_json(js_value)
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toJson))]
    pub fn to_json(&self) -> Result<JsValue, String> {
        self.export_json()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn device(&self) -> Option<String> {
        self.device.clone()
    }
}

#[cfg(target_arch = "wasm32")]
impl Json for MapkyAppSequence {}

impl TimestampId for MapkyAppSequence {}

impl HasIdPath for MapkyAppSequence {
    const PATH_SEGMENT: &'static str = "sequences/";

    fn create_path(id: &str) -> String {
        [PUBLIC_PATH, MAPKY_PATH, Self::PATH_SEGMENT, id].concat()
    }
}

impl Validatable for MapkyAppSequence {
    fn sanitize(self) -> Self {
        let name = self.name.map(|n| n.trim().to_string());
        let description = self.description.map(|d| d.trim().to_string());
        let device = self.device.map(|d| d.trim().to_string());

        MapkyAppSequence {
            name,
            description,
            device,
            ..self
        }
    }

    fn validate(&self, id: Option<&str>) -> Result<(), String> {
        if let Some(id) = id {
            self.validate_id(id)?;
        }

        if let Some(ref name) = self.name {
            if name.chars().count() > MAX_SEQUENCE_NAME_LENGTH {
                return Err(format!(
                    "Validation Error: Sequence name exceeds maximum length of {} characters",
                    MAX_SEQUENCE_NAME_LENGTH
                ));
            }
        }

        if let Some(ref desc) = self.description {
            if desc.chars().count() > MAX_SEQUENCE_DESCRIPTION_LENGTH {
                return Err(format!(
                    "Validation Error: Sequence description exceeds maximum length of {} characters",
                    MAX_SEQUENCE_DESCRIPTION_LENGTH
                ));
            }
        }

        if let Some(ref device) = self.device {
            if device.chars().count() > MAX_SEQUENCE_DEVICE_LENGTH {
                return Err(format!(
                    "Validation Error: Sequence device exceeds maximum length of {} characters",
                    MAX_SEQUENCE_DEVICE_LENGTH
                ));
            }
        }

        validate_timestamp_microseconds(self.captured_at_start, "captured_at_start")?;
        validate_timestamp_microseconds(self.captured_at_end, "captured_at_end")?;
        if self.captured_at_start > self.captured_at_end {
            return Err(format!(
                "Validation Error: captured_at_start ({}) must be <= captured_at_end ({})",
                self.captured_at_start, self.captured_at_end
            ));
        }

        if self.capture_count > MAX_SEQUENCE_CAPTURE_COUNT {
            return Err(format!(
                "Validation Error: capture_count {} exceeds maximum of {}",
                self.capture_count, MAX_SEQUENCE_CAPTURE_COUNT
            ));
        }

        if let Some(ref bbox) = self.bbox {
            bbox.validate()?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> MapkyAppSequence {
        let now = crate::common::timestamp();
        MapkyAppSequence::new(GeoCaptureKind::Photo, now - 1_000_000, now, 3)
    }

    #[test]
    fn test_create_id_and_path() {
        let seq = base();
        let id = seq.create_id();
        assert_eq!(id.len(), 13);
        let path = MapkyAppSequence::create_path(&id);
        assert!(path.starts_with("/pub/mapky.app/sequences/"));
    }

    #[test]
    fn test_validate_happy() {
        let seq = base();
        let id = seq.create_id();
        assert!(seq.validate(Some(&id)).is_ok());
    }

    #[test]
    fn test_validate_start_after_end() {
        let now = crate::common::timestamp();
        let seq = MapkyAppSequence::new(GeoCaptureKind::Panorama, now, now - 1_000_000, 2);
        let id = seq.create_id();
        let result = seq.validate(Some(&id));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("captured_at_start"));
    }

    #[test]
    fn test_validate_non_positive_start() {
        let seq = MapkyAppSequence::new(GeoCaptureKind::Photo, 0, 1, 1);
        let id = seq.create_id();
        assert!(seq.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_validate_name_too_long() {
        let mut seq = base();
        seq.name = Some("a".repeat(MAX_SEQUENCE_NAME_LENGTH + 1));
        let id = seq.create_id();
        assert!(seq.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_validate_description_too_long() {
        let mut seq = base();
        seq.description = Some("a".repeat(MAX_SEQUENCE_DESCRIPTION_LENGTH + 1));
        let id = seq.create_id();
        assert!(seq.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_validate_device_too_long() {
        let mut seq = base();
        seq.device = Some("a".repeat(MAX_SEQUENCE_DEVICE_LENGTH + 1));
        let id = seq.create_id();
        assert!(seq.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_validate_capture_count_cap() {
        let mut seq = base();
        seq.capture_count = MAX_SEQUENCE_CAPTURE_COUNT + 1;
        let id = seq.create_id();
        assert!(seq.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_validate_bbox_inverted() {
        let mut seq = base();
        seq.bbox = Some(BoundingBox {
            min_lat: 10.0,
            min_lon: 0.0,
            max_lat: 5.0,
            max_lon: 10.0,
        });
        let id = seq.create_id();
        assert!(seq.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_validate_bbox_out_of_range() {
        let mut seq = base();
        seq.bbox = Some(BoundingBox {
            min_lat: -95.0,
            min_lon: 0.0,
            max_lat: 0.0,
            max_lon: 10.0,
        });
        let id = seq.create_id();
        assert!(seq.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_serde_roundtrip() {
        let mut seq = base();
        seq.name = Some("Street walk".into());
        seq.device = Some("iPhone 15 Pro".into());
        seq.bbox = Some(BoundingBox::new(47.0, 8.0, 47.1, 8.1));
        let json = serde_json::to_string(&seq).unwrap();
        assert!(json.contains("\"photo\""));
        let parsed: MapkyAppSequence = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.name, Some("Street walk".into()));
        assert_eq!(parsed.device, Some("iPhone 15 Pro".into()));
        assert!(parsed.bbox.is_some());
    }
}
