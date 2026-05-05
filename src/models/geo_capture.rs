use crate::{
    common::sanitize_url,
    constants::MAX_CAPTION_LENGTH,
    traits::{HasIdPath, TimestampId, Validatable},
    validation::{
        validate_coordinates, validate_heading, validate_pubky_uri, validate_sequence_uri,
        validate_timestamp_microseconds,
    },
    MAPKY_PATH, PUBLIC_PATH,
};
use serde::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
use crate::traits::Json;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Rendering hint for geo-captured media
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum GeoCaptureKind {
    #[default]
    Photo,
    Panorama,
    Video,
    Video360,
    Model3d,
    PointCloud,
    Audio,
    Other,
}

/// Geo-located media capture (street-level imagery, panoramas, 3D models, etc.)
/// URI: /pub/mapky.app/geo_captures/:capture_id
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct MapkyAppGeoCapture {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub file_uri: String,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub kind: GeoCaptureKind,
    pub lat: f64,
    pub lon: f64,
    pub ele: Option<f64>,
    pub heading: Option<f64>,
    pub pitch: Option<f64>,
    pub fov: Option<f64>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub caption: Option<String>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub sequence_uri: Option<String>,
    pub sequence_index: Option<u32>,
    /// Moment the media was captured (UNIX microseconds). Distinct from the
    /// TimestampId, which reflects when the record was created/uploaded.
    pub captured_at: Option<i64>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl MapkyAppGeoCapture {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(file_uri: String, kind: GeoCaptureKind, lat: f64, lon: f64) -> Self {
        let capture = MapkyAppGeoCapture {
            file_uri,
            kind,
            lat,
            lon,
            ele: None,
            heading: None,
            pitch: None,
            fov: None,
            caption: None,
            sequence_uri: None,
            sequence_index: None,
            captured_at: None,
        };
        capture.sanitize()
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl MapkyAppGeoCapture {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromJson))]
    pub fn from_json(js_value: &JsValue) -> Result<Self, String> {
        Self::import_json(js_value)
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toJson))]
    pub fn to_json(&self) -> Result<JsValue, String> {
        self.export_json()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn file_uri(&self) -> String {
        self.file_uri.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn caption(&self) -> Option<String> {
        self.caption.clone()
    }
}

#[cfg(target_arch = "wasm32")]
impl Json for MapkyAppGeoCapture {}

impl TimestampId for MapkyAppGeoCapture {}

impl HasIdPath for MapkyAppGeoCapture {
    const PATH_SEGMENT: &'static str = "geo_captures/";

    fn create_path(id: &str) -> String {
        [PUBLIC_PATH, MAPKY_PATH, Self::PATH_SEGMENT, id].concat()
    }
}

impl Validatable for MapkyAppGeoCapture {
    fn sanitize(self) -> Self {
        let file_uri = sanitize_url(&self.file_uri);
        let caption = self.caption.map(|c| c.trim().to_string());
        let sequence_uri = self.sequence_uri.map(|u| sanitize_url(&u));

        MapkyAppGeoCapture {
            file_uri,
            caption,
            sequence_uri,
            ..self
        }
    }

    fn validate(&self, id: Option<&str>) -> Result<(), String> {
        if let Some(id) = id {
            self.validate_id(id)?;
        }

        // Validate file_uri
        validate_pubky_uri(&self.file_uri)?;

        // Validate coordinates
        validate_coordinates(self.lat, self.lon)?;

        // Validate heading
        if let Some(heading) = self.heading {
            validate_heading(heading)?;
        }

        // Validate pitch (-90 to 90)
        if let Some(pitch) = self.pitch {
            if !(-90.0..=90.0).contains(&pitch) {
                return Err(format!(
                    "Validation Error: Pitch {} out of range (-90 to 90)",
                    pitch
                ));
            }
        }

        // Validate FOV (1 to 360)
        if let Some(fov) = self.fov {
            if !(1.0..=360.0).contains(&fov) {
                return Err(format!(
                    "Validation Error: FOV {} out of range (1 to 360)",
                    fov
                ));
            }
        }

        // Validate caption
        if let Some(ref caption) = self.caption {
            if caption.chars().count() > MAX_CAPTION_LENGTH {
                return Err(format!(
                    "Validation Error: Caption exceeds maximum length of {} characters",
                    MAX_CAPTION_LENGTH
                ));
            }
        }

        // Validate sequence pair (both or neither)
        if self.sequence_uri.is_some() != self.sequence_index.is_some() {
            return Err(
                "Validation Error: sequence_uri and sequence_index must both be present or both absent".into(),
            );
        }

        // Validate sequence URI if present — must point at a /pub/mapky.app/sequences/ resource
        if let Some(ref uri) = self.sequence_uri {
            validate_sequence_uri(uri)
                .map_err(|e| format!("Validation Error: Invalid sequence URI: {}", e))?;
        }

        // Validate captured_at if present
        if let Some(captured_at) = self.captured_at {
            validate_timestamp_microseconds(captured_at, "captured_at")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_id() {
        let capture = MapkyAppGeoCapture::new(
            "pubky://user123/pub/mapky.app/files/0034A0X7NJ52G".into(),
            GeoCaptureKind::Photo,
            47.3769,
            8.5417,
        );
        let id = capture.create_id();
        assert_eq!(id.len(), 13);
    }

    #[test]
    fn test_create_path() {
        let capture = MapkyAppGeoCapture::new(
            "pubky://user123/pub/mapky.app/files/0034A0X7NJ52G".into(),
            GeoCaptureKind::Panorama,
            0.0,
            0.0,
        );
        let id = capture.create_id();
        let path = MapkyAppGeoCapture::create_path(&id);
        assert!(path.starts_with("/pub/mapky.app/geo_captures/"));
    }

    #[test]
    fn test_validate_happy() {
        let capture = MapkyAppGeoCapture::new(
            "pubky://user123/pub/mapky.app/files/0034A0X7NJ52G".into(),
            GeoCaptureKind::Photo,
            47.3769,
            8.5417,
        );
        let id = capture.create_id();
        assert!(capture.validate(Some(&id)).is_ok());
    }

    #[test]
    fn test_validate_invalid_coordinates() {
        let capture = MapkyAppGeoCapture::new(
            "pubky://user123/pub/mapky.app/files/0034A0X7NJ52G".into(),
            GeoCaptureKind::Photo,
            91.0,
            0.0,
        );
        let id = capture.create_id();
        assert!(capture.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_validate_invalid_pitch() {
        let mut capture = MapkyAppGeoCapture::new(
            "pubky://user123/pub/mapky.app/files/0034A0X7NJ52G".into(),
            GeoCaptureKind::Photo,
            0.0,
            0.0,
        );
        capture.pitch = Some(91.0);
        let id = capture.create_id();
        assert!(capture.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_validate_sequence_pair() {
        let mut capture = MapkyAppGeoCapture::new(
            "pubky://user123/pub/mapky.app/files/0034A0X7NJ52G".into(),
            GeoCaptureKind::Photo,
            0.0,
            0.0,
        );
        // Only sequence_uri without index
        capture.sequence_uri = Some("pubky://user123/pub/mapky.app/sequences/0034A0X7NJ52G".into());
        let id = capture.create_id();
        let result = capture.validate(Some(&id));
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("sequence_uri and sequence_index"));
    }

    #[test]
    fn test_validate_sequence_pair_both_present() {
        let mut capture = MapkyAppGeoCapture::new(
            "pubky://user123/pub/mapky.app/files/0034A0X7NJ52G".into(),
            GeoCaptureKind::Photo,
            0.0,
            0.0,
        );
        capture.sequence_uri = Some("pubky://user123/pub/mapky.app/sequences/0034A0X7NJ52G".into());
        capture.sequence_index = Some(0);
        let id = capture.create_id();
        assert!(capture.validate(Some(&id)).is_ok());
    }

    #[test]
    fn test_validate_sequence_uri_must_be_sequences_path() {
        let mut capture = MapkyAppGeoCapture::new(
            "pubky://user123/pub/mapky.app/files/0034A0X7NJ52G".into(),
            GeoCaptureKind::Photo,
            0.0,
            0.0,
        );
        // sequence_uri pointing at a route — now rejected
        capture.sequence_uri = Some("pubky://user123/pub/mapky.app/routes/0034A0X7NJ52G".into());
        capture.sequence_index = Some(0);
        let id = capture.create_id();
        let result = capture.validate(Some(&id));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("/pub/mapky.app/sequences/"));
    }

    #[test]
    fn test_validate_captured_at_ok() {
        let mut capture = MapkyAppGeoCapture::new(
            "pubky://user123/pub/mapky.app/files/0034A0X7NJ52G".into(),
            GeoCaptureKind::Photo,
            0.0,
            0.0,
        );
        capture.captured_at = Some(crate::common::timestamp());
        let id = capture.create_id();
        assert!(capture.validate(Some(&id)).is_ok());
    }

    #[test]
    fn test_validate_captured_at_non_positive() {
        let mut capture = MapkyAppGeoCapture::new(
            "pubky://user123/pub/mapky.app/files/0034A0X7NJ52G".into(),
            GeoCaptureKind::Photo,
            0.0,
            0.0,
        );
        capture.captured_at = Some(0);
        let id = capture.create_id();
        assert!(capture.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_validate_captured_at_too_far_future() {
        let mut capture = MapkyAppGeoCapture::new(
            "pubky://user123/pub/mapky.app/files/0034A0X7NJ52G".into(),
            GeoCaptureKind::Photo,
            0.0,
            0.0,
        );
        // 10 days in the future
        capture.captured_at = Some(crate::common::timestamp() + 10 * 86_400_000_000);
        let id = capture.create_id();
        assert!(capture.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_validate_non_pubky_file_uri() {
        let capture = MapkyAppGeoCapture {
            file_uri: "https://example.com/file.jpg".into(),
            kind: GeoCaptureKind::Photo,
            lat: 0.0,
            lon: 0.0,
            ..Default::default()
        };
        let id = capture.create_id();
        assert!(capture.validate(Some(&id)).is_err());
    }

    #[test]
    fn test_all_capture_kinds() {
        let kinds = vec![
            GeoCaptureKind::Photo,
            GeoCaptureKind::Panorama,
            GeoCaptureKind::Video,
            GeoCaptureKind::Video360,
            GeoCaptureKind::Model3d,
            GeoCaptureKind::PointCloud,
            GeoCaptureKind::Audio,
            GeoCaptureKind::Other,
        ];
        for kind in kinds {
            let capture = MapkyAppGeoCapture::new(
                "pubky://user123/pub/mapky.app/files/0034A0X7NJ52G".into(),
                kind,
                0.0,
                0.0,
            );
            let id = capture.create_id();
            assert!(capture.validate(Some(&id)).is_ok());
        }
    }

    #[test]
    fn test_serde_roundtrip() {
        let capture = MapkyAppGeoCapture::new(
            "pubky://user123/pub/mapky.app/files/0034A0X7NJ52G".into(),
            GeoCaptureKind::Model3d,
            47.3769,
            8.5417,
        );
        let json = serde_json::to_string(&capture).unwrap();
        // rename_all = "snake_case" produces "model3d" for Model3d
        assert!(json.contains("\"model3d\""));
        let parsed: MapkyAppGeoCapture = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.kind, GeoCaptureKind::Model3d);
    }
}
